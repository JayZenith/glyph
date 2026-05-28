#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    StartWork,
    Resolve,
    Close,
    Reopen,
    Cancel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub status: Status,
    pub assignee: Option<&'static str>,
    pub resolution: Option<&'static str>,
    pub closed_at: Option<u32>,
    pub revision: u32,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::Open,
            assignee: None,
            resolution: None,
            closed_at: None,
            revision: 0,
        }
    }

    pub fn apply(&mut self, event: Event, now: u32) {
        match event {
            Event::StartWork => {
                if self.status == Status::Open {
                    self.status = Status::InProgress;
                    if self.assignee.is_none() {
                        self.assignee = Some("system");
                    }
                }
            }
            Event::Resolve => {
                if matches!(self.status, Status::Open | Status::InProgress) {
                    self.status = Status::Resolved;
                    if self.resolution.is_none() {
                        self.resolution = Some("fixed");
                    }
                }
            }
            Event::Close => {
                if self.status == Status::Resolved {
                    self.status = Status::Closed;
                    self.closed_at = Some(now);
                }
            }
            Event::Reopen => {
                if matches!(self.status, Status::Resolved | Status::Closed | Status::Cancelled) {
                    self.status = Status::Open;
                    self.revision += 1;
                }
            }
            Event::Cancel => {
                self.status = Status::Cancelled;
                self.closed_at = Some(now);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reopen_from_closed_clears_terminal_fields_and_bumps_revision() {
        let mut t = Ticket::new();
        t.assignee = Some("alice");
        t.apply(Event::StartWork, 1);
        t.apply(Event::Resolve, 2);
        t.apply(Event::Close, 7);

        assert_eq!(t.status, Status::Closed);
        assert_eq!(t.closed_at, Some(7));
        assert_eq!(t.resolution, Some("fixed"));

        t.apply(Event::Reopen, 10);
        assert_eq!(t.status, Status::Open);
        assert_eq!(t.revision, 1);
        assert_eq!(t.closed_at, None);
        assert_eq!(t.resolution, None);
        assert_eq!(t.assignee, Some("alice"));
    }

    #[test]
    fn reopen_from_resolved_keeps_not_closed_but_clears_resolution() {
        let mut t = Ticket::new();
        t.apply(Event::Resolve, 3);
        assert_eq!(t.status, Status::Resolved);
        assert_eq!(t.closed_at, None);
        assert_eq!(t.resolution, Some("fixed"));

        t.apply(Event::Reopen, 4);
        assert_eq!(t.status, Status::Open);
        assert_eq!(t.revision, 1);
        assert_eq!(t.closed_at, None);
        assert_eq!(t.resolution, None);
    }

    #[test]
    fn cancel_is_terminal_and_does_not_allow_reopen_or_startwork() {
        let mut t = Ticket::new();
        t.apply(Event::StartWork, 1);
        t.apply(Event::Cancel, 2);

        let snapshot = t.clone();
        t.apply(Event::Reopen, 3);
        assert_eq!(t, snapshot);

        t.apply(Event::StartWork, 4);
        assert_eq!(t, snapshot);
    }

    #[test]
    fn cancel_from_resolved_clears_resolution_and_sets_closed_time() {
        let mut t = Ticket::new();
        t.apply(Event::Resolve, 5);
        t.apply(Event::Cancel, 9);

        assert_eq!(t.status, Status::Cancelled);
        assert_eq!(t.closed_at, Some(9));
        assert_eq!(t.resolution, None);
    }
}
