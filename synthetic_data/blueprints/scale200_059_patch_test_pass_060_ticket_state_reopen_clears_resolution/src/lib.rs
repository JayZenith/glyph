#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    New,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Start,
    Resolve(&'static str),
    Close,
    Reopen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub status: Status,
    pub resolution: Option<&'static str>,
    pub reopen_count: u32,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::New,
            resolution: None,
            reopen_count: 0,
        }
    }

    pub fn apply(&mut self, event: Event) {
        match event {
            Event::Start => {
                if self.status == Status::New {
                    self.status = Status::InProgress;
                }
            }
            Event::Resolve(reason) => {
                if self.status == Status::InProgress {
                    self.status = Status::Resolved;
                    self.resolution = Some(reason);
                }
            }
            Event::Close => {
                if self.status == Status::Resolved {
                    self.status = Status::Closed;
                }
            }
            Event::Reopen => {
                if self.status == Status::Resolved || self.status == Status::Closed {
                    self.status = Status::Resolved;
                    self.reopen_count += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reopening_resolved_ticket_returns_to_in_progress_and_clears_resolution() {
        let mut ticket = Ticket::new();
        ticket.apply(Event::Start);
        ticket.apply(Event::Resolve("fixed"));

        ticket.apply(Event::Reopen);

        assert_eq!(ticket.status, Status::InProgress);
        assert_eq!(ticket.resolution, None);
        assert_eq!(ticket.reopen_count, 1);
    }

    #[test]
    fn reopening_closed_ticket_allows_work_to_resume() {
        let mut ticket = Ticket::new();
        ticket.apply(Event::Start);
        ticket.apply(Event::Resolve("done"));
        ticket.apply(Event::Close);

        ticket.apply(Event::Reopen);

        assert_eq!(ticket.status, Status::InProgress);
        assert_eq!(ticket.resolution, None);
        assert_eq!(ticket.reopen_count, 1);
    }

    #[test]
    fn reopen_from_new_has_no_effect() {
        let mut ticket = Ticket::new();
        ticket.apply(Event::Reopen);

        assert_eq!(ticket.status, Status::New);
        assert_eq!(ticket.resolution, None);
        assert_eq!(ticket.reopen_count, 0);
    }
}
