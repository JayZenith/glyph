#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub status: Status,
    pub assignee: Option<String>,
    pub resolution: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Assign(&'static str),
    Start,
    Resolve(&'static str),
    Close,
    Reopen,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::Open,
            assignee: None,
            resolution: None,
        }
    }

    pub fn apply(&mut self, event: Event) {
        match event {
            Event::Assign(name) => {
                self.assignee = Some(name.to_string());
                if matches!(self.status, Status::Open) {
                    self.status = Status::InProgress;
                }
            }
            Event::Start => {
                if self.assignee.is_some() && matches!(self.status, Status::Open) {
                    self.status = Status::InProgress;
                }
            }
            Event::Resolve(note) => {
                if matches!(self.status, Status::InProgress) {
                    self.status = Status::Resolved;
                    self.resolution = Some(note.to_string());
                }
            }
            Event::Close => {
                if matches!(self.status, Status::Resolved) {
                    self.status = Status::Closed;
                }
            }
            Event::Reopen => {
                if matches!(self.status, Status::Resolved | Status::Closed) {
                    self.status = Status::Open;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reopen_clears_resolution_and_uncloses() {
        let mut t = Ticket::new();
        t.apply(Event::Assign("ana"));
        t.apply(Event::Resolve("done"));
        t.apply(Event::Close);
        t.apply(Event::Reopen);

        assert_eq!(t.status, Status::Open);
        assert_eq!(t.assignee.as_deref(), Some("ana"));
        assert_eq!(t.resolution, None);
    }

    #[test]
    fn reassigning_closed_ticket_reopens_work() {
        let mut t = Ticket::new();
        t.apply(Event::Assign("ana"));
        t.apply(Event::Resolve("fixed"));
        t.apply(Event::Close);
        t.apply(Event::Assign("ben"));

        assert_eq!(t.status, Status::InProgress);
        assert_eq!(t.assignee.as_deref(), Some("ben"));
        assert_eq!(t.resolution, None);
    }

    #[test]
    fn start_requires_assignee() {
        let mut t = Ticket::new();
        t.apply(Event::Start);
        assert_eq!(t.status, Status::Open);
    }
}
