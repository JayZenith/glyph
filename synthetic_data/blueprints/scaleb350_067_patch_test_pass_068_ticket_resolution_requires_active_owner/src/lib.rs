#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Blocked,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Assign(String),
    Start,
    Block,
    Unblock,
    Resolve,
    Reopen,
    Unassign,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub status: Status,
    pub owner: Option<String>,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::Open,
            owner: None,
        }
    }

    pub fn apply(&mut self, event: Event) {
        match event {
            Event::Assign(name) => {
                self.owner = Some(name);
                if self.status == Status::Open {
                    self.status = Status::InProgress;
                }
            }
            Event::Start => {
                if self.owner.is_some() && self.status != Status::Resolved {
                    self.status = Status::InProgress;
                }
            }
            Event::Block => {
                if self.status == Status::InProgress {
                    self.status = Status::Blocked;
                }
            }
            Event::Unblock => {
                if self.status == Status::Blocked {
                    self.status = Status::InProgress;
                }
            }
            Event::Resolve => {
                if self.owner.is_some() {
                    self.status = Status::Resolved;
                }
            }
            Event::Reopen => {
                if self.status == Status::Resolved {
                    self.status = Status::Open;
                }
            }
            Event::Unassign => {
                self.owner = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_resolve_without_owner() {
        let mut t = Ticket::new();
        t.apply(Event::Resolve);
        assert_eq!(t.status, Status::Open);
    }

    #[test]
    fn cannot_resolve_when_blocked() {
        let mut t = Ticket::new();
        t.apply(Event::Assign("maya".into()));
        t.apply(Event::Block);
        t.apply(Event::Resolve);
        assert_eq!(t.status, Status::Blocked);
    }

    #[test]
    fn unassigning_from_in_progress_returns_to_open() {
        let mut t = Ticket::new();
        t.apply(Event::Assign("maya".into()));
        t.apply(Event::Unassign);
        assert_eq!(t.owner, None);
        assert_eq!(t.status, Status::Open);
    }

    #[test]
    fn unassigning_blocked_ticket_keeps_blocked_state() {
        let mut t = Ticket::new();
        t.apply(Event::Assign("maya".into()));
        t.apply(Event::Block);
        t.apply(Event::Unassign);
        assert_eq!(t.owner, None);
        assert_eq!(t.status, Status::Blocked);
    }

    #[test]
    fn reopen_then_start_with_owner_moves_back_to_in_progress() {
        let mut t = Ticket::new();
        t.apply(Event::Assign("maya".into()));
        t.apply(Event::Resolve);
        t.apply(Event::Reopen);
        assert_eq!(t.status, Status::Open);
        t.apply(Event::Start);
        assert_eq!(t.status, Status::InProgress);
    }
}
