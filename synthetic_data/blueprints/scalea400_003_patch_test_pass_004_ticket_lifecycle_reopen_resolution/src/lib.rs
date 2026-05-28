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

    pub fn apply(&mut self, event: Event, actor: Option<&'static str>, now: u64) {
        match event {
            Event::Start => {
                if self.status == Status::Open {
                    self.status = Status::InProgress;
                    self.assignee = actor;
                }
            }
            Event::Resolve => {
                if self.status == Status::InProgress {
                    self.status = Status::Resolved;
                    self.resolution = Some("done");
                }
            }
            Event::Close => {
                if self.status == Status::Resolved {
                    self.status = Status::Closed;
                    self.closed_at = Some(now);
                }
            }
            Event::Reopen => {
                if self.status == Status::Closed {
                    self.status = Status::Open;
                }
            }
            Event::Cancel => {
                self.status = Status::Closed;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, Status, Ticket};

    #[test]
    fn reopen_closed_ticket_clears_terminal_fields_and_returns_to_open() {
        let mut ticket = Ticket::new();
        ticket.apply(Event::Start, Some("alice"), 10);
        ticket.apply(Event::Resolve, Some("alice"), 20);
        ticket.apply(Event::Close, Some("alice"), 30);

        assert_eq!(ticket.status, Status::Closed);
        assert_eq!(ticket.resolution, Some("done"));
        assert_eq!(ticket.closed_at, Some(30));

        ticket.apply(Event::Reopen, Some("bob"), 40);

        assert_eq!(ticket.status, Status::Open);
        assert_eq!(ticket.assignee, None);
        assert_eq!(ticket.resolution, None);
        assert_eq!(ticket.closed_at, None);
    }

    #[test]
    fn cancel_only_works_before_closure_and_clears_context() {
        let mut ticket = Ticket::new();
        ticket.apply(Event::Start, Some("alice"), 1);
        ticket.apply(Event::Cancel, Some("alice"), 2);

        assert_eq!(ticket.status, Status::Closed);
        assert_eq!(ticket.assignee, None);
        assert_eq!(ticket.resolution, None);
        assert_eq!(ticket.closed_at, Some(2));

        let snapshot = ticket.clone();
        ticket.apply(Event::Cancel, Some("bob"), 3);
        assert_eq!(ticket, snapshot);
    }

    #[test]
    fn start_from_resolved_or_closed_does_not_change_ticket() {
        let mut resolved = Ticket::new();
        resolved.apply(Event::Start, Some("alice"), 1);
        resolved.apply(Event::Resolve, Some("alice"), 2);
        let before_resolved = resolved.clone();
        resolved.apply(Event::Start, Some("bob"), 3);
        assert_eq!(resolved, before_resolved);

        let mut closed = Ticket::new();
        closed.apply(Event::Start, Some("alice"), 1);
        closed.apply(Event::Resolve, Some("alice"), 2);
        closed.apply(Event::Close, Some("alice"), 3);
        let before_closed = closed.clone();
        closed.apply(Event::Start, Some("bob"), 4);
        assert_eq!(closed, before_closed);
    }

    #[test]
    fn closing_requires_resolution_but_preserves_resolution_data() {
        let mut ticket = Ticket::new();
        ticket.apply(Event::Close, Some("alice"), 5);
        assert_eq!(ticket.status, Status::Open);
        assert_eq!(ticket.closed_at, None);

        ticket.apply(Event::Start, Some("alice"), 6);
        ticket.apply(Event::Resolve, Some("alice"), 7);
        ticket.apply(Event::Close, Some("alice"), 8);

        assert_eq!(ticket.status, Status::Closed);
        assert_eq!(ticket.resolution, Some("done"));
        assert_eq!(ticket.closed_at, Some(8));
    }
}
