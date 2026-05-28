#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
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
    pub closed_at: Option<u64>,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::Open,
            assignee: None,
            resolution: None,
            closed_at: None,
        }
    }

    pub fn apply(&mut self, event: Event, ts: u64) {
        match event {
            Event::Start => {
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
                    self.closed_at = Some(ts);
                }
            }
            Event::Reopen => {
                if matches!(self.status, Status::Resolved | Status::Closed) {
                    self.status = Status::Open;
                }
            }
            Event::Cancel => {
                self.assignee = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reopen_from_resolved_clears_resolution_and_keeps_closed_at_empty() {
        let mut t = Ticket::new();
        t.apply(Event::Start, 1);
        t.apply(Event::Resolve, 2);
        assert_eq!(t.status, Status::Resolved);
        assert_eq!(t.resolution, Some("fixed"));
        assert_eq!(t.closed_at, None);

        t.apply(Event::Reopen, 3);
        assert_eq!(t.status, Status::Open);
        assert_eq!(t.resolution, None);
        assert_eq!(t.closed_at, None);
    }

    #[test]
    fn reopen_from_closed_clears_resolution_and_closed_timestamp() {
        let mut t = Ticket::new();
        t.apply(Event::Resolve, 10);
        t.apply(Event::Close, 11);
        assert_eq!(t.status, Status::Closed);
        assert_eq!(t.resolution, Some("fixed"));
        assert_eq!(t.closed_at, Some(11));

        t.apply(Event::Reopen, 12);
        assert_eq!(t.status, Status::Open);
        assert_eq!(t.resolution, None);
        assert_eq!(t.closed_at, None);
    }

    #[test]
    fn cancel_only_clears_assignee_while_active() {
        let mut t = Ticket::new();
        t.assignee = Some("alex");
        t.apply(Event::Resolve, 5);
        assert_eq!(t.status, Status::Resolved);
        assert_eq!(t.assignee, Some("alex"));

        t.apply(Event::Cancel, 6);
        assert_eq!(t.assignee, Some("alex"));
        assert_eq!(t.status, Status::Resolved);
    }

    #[test]
    fn cancel_clears_assignee_in_open_or_in_progress() {
        let mut open = Ticket::new();
        open.assignee = Some("sam");
        open.apply(Event::Cancel, 1);
        assert_eq!(open.assignee, None);
        assert_eq!(open.status, Status::Open);

        let mut active = Ticket::new();
        active.assignee = Some("pat");
        active.apply(Event::Start, 2);
        active.apply(Event::Cancel, 3);
        assert_eq!(active.assignee, None);
        assert_eq!(active.status, Status::InProgress);
    }
}
