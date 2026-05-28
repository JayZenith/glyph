#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    InProgress,
    Blocked,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Block,
    Unblock,
    Resolve,
    Close,
    Reopen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub status: Status,
    pub assignee: Option<&'static str>,
    pub resolution: Option<&'static str>,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::New,
            assignee: None,
            resolution: None,
        }
    }

    pub fn apply(&mut self, event: Event) {
        match event {
            Event::Start => {
                if self.status == Status::New || self.status == Status::Blocked {
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
                    self.status = Status::New;
                }
            }
            Event::Resolve => {
                if self.status == Status::InProgress || self.status == Status::Blocked {
                    self.status = Status::Resolved;
                    if self.resolution.is_none() {
                        self.resolution = Some("done");
                    }
                }
            }
            Event::Close => {
                if self.status == Status::Resolved {
                    self.status = Status::Closed;
                }
            }
            Event::Reopen => {
                if self.status == Status::Closed || self.status == Status::Resolved {
                    self.status = Status::New;
                }
            }
        }
    }
}

pub fn replay(events: &[Event], assignee: Option<&'static str>) -> Ticket {
    let mut ticket = Ticket::new();
    ticket.assignee = assignee;
    for &e in events {
        ticket.apply(e);
    }
    ticket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unblock_returns_to_in_progress_not_new() {
        let t = replay(&[Event::Start, Event::Block, Event::Unblock], Some("dev"));
        assert_eq!(t.status, Status::InProgress);
        assert_eq!(t.assignee, Some("dev"));
        assert_eq!(t.resolution, None);
    }

    #[test]
    fn resolve_requires_active_work_not_blocked() {
        let t = replay(&[Event::Start, Event::Block, Event::Resolve], None);
        assert_eq!(t.status, Status::Blocked);
        assert_eq!(t.resolution, None);
    }

    #[test]
    fn reopening_from_resolved_goes_back_to_in_progress_and_clears_resolution() {
        let t = replay(&[Event::Start, Event::Resolve, Event::Reopen], Some("ops"));
        assert_eq!(t.status, Status::InProgress);
        assert_eq!(t.assignee, Some("ops"));
        assert_eq!(t.resolution, None);
    }

    #[test]
    fn reopening_from_closed_goes_to_new_and_clears_resolution() {
        let t = replay(&[Event::Start, Event::Resolve, Event::Close, Event::Reopen], None);
        assert_eq!(t.status, Status::New);
        assert_eq!(t.resolution, None);
    }

    #[test]
    fn start_does_not_restart_closed_ticket() {
        let t = replay(&[Event::Start, Event::Resolve, Event::Close, Event::Start], None);
        assert_eq!(t.status, Status::Closed);
    }
}
