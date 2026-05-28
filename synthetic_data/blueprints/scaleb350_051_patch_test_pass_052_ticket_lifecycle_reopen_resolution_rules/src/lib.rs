#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    StartWork,
    Resolve,
    Close,
    Reopen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub status: Status,
    pub resolution: Option<&'static str>,
    pub reopen_count: u8,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::Open,
            resolution: None,
            reopen_count: 0,
        }
    }

    pub fn apply(&mut self, event: Event) {
        match event {
            Event::StartWork => {
                if self.status == Status::Open {
                    self.status = Status::InProgress;
                }
            }
            Event::Resolve => {
                if matches!(self.status, Status::Open | Status::InProgress) {
                    self.status = Status::Resolved;
                    self.resolution = Some("fixed");
                }
            }
            Event::Close => {
                if self.status == Status::Resolved {
                    self.status = Status::Closed;
                }
            }
            Event::Reopen => {
                if matches!(self.status, Status::Resolved | Status::Closed) {
                    self.status = Status::Open;
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
    fn close_clears_resolution_and_preserves_reopen_count() {
        let mut t = Ticket::new();
        t.apply(Event::StartWork);
        t.apply(Event::Resolve);
        assert_eq!(t.resolution, Some("fixed"));
        t.apply(Event::Close);
        assert_eq!(t.status, Status::Closed);
        assert_eq!(t.resolution, None);
        assert_eq!(t.reopen_count, 0);
    }

    #[test]
    fn reopening_closed_ticket_goes_to_in_progress_and_clears_resolution() {
        let mut t = Ticket::new();
        t.apply(Event::Resolve);
        t.apply(Event::Close);
        t.apply(Event::Reopen);
        assert_eq!(t.status, Status::InProgress);
        assert_eq!(t.resolution, None);
        assert_eq!(t.reopen_count, 1);
    }

    #[test]
    fn reopening_resolved_ticket_returns_to_open_and_clears_resolution() {
        let mut t = Ticket::new();
        t.apply(Event::StartWork);
        t.apply(Event::Resolve);
        t.apply(Event::Reopen);
        assert_eq!(t.status, Status::Open);
        assert_eq!(t.resolution, None);
        assert_eq!(t.reopen_count, 1);
    }

    #[test]
    fn repeated_reopen_cycles_accumulate_and_allow_rework() {
        let mut t = Ticket::new();
        t.apply(Event::Resolve);
        t.apply(Event::Close);
        t.apply(Event::Reopen);
        t.apply(Event::Resolve);
        t.apply(Event::Close);
        t.apply(Event::Reopen);
        assert_eq!(t.status, Status::InProgress);
        assert_eq!(t.reopen_count, 2);
        assert_eq!(t.resolution, None);
    }

    #[test]
    fn invalid_events_do_not_mutate_state() {
        let mut t = Ticket::new();
        t.apply(Event::Close);
        assert_eq!(t.status, Status::Open);
        assert_eq!(t.resolution, None);
        assert_eq!(t.reopen_count, 0);

        t.apply(Event::Reopen);
        assert_eq!(t.status, Status::Open);
        assert_eq!(t.resolution, None);
        assert_eq!(t.reopen_count, 0);
    }
}
