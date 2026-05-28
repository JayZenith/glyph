#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    OnHold,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Pause,
    Resume,
    Resolve,
    Close,
    Reopen,
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

    pub fn apply(&mut self, event: Event, actor: Option<&'static str>, now: u64) -> bool {
        match event {
            Event::Start => {
                if self.status == Status::Open || self.status == Status::OnHold {
                    self.status = Status::InProgress;
                    self.assignee = actor.or(self.assignee);
                    true
                } else {
                    false
                }
            }
            Event::Pause => {
                if self.status == Status::InProgress {
                    self.status = Status::OnHold;
                    true
                } else {
                    false
                }
            }
            Event::Resume => {
                if self.status == Status::OnHold {
                    self.status = Status::InProgress;
                    true
                } else {
                    false
                }
            }
            Event::Resolve => {
                if self.status == Status::InProgress {
                    self.status = Status::Resolved;
                    self.resolution = Some("fixed");
                    true
                } else {
                    false
                }
            }
            Event::Close => {
                if self.status == Status::Resolved {
                    self.status = Status::Closed;
                    self.closed_at = Some(now);
                    true
                } else {
                    false
                }
            }
            Event::Reopen => {
                if self.status == Status::Resolved || self.status == Status::Closed {
                    self.status = Status::Open;
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reopen_clears_terminal_metadata_and_returns_to_open() {
        let mut t = Ticket::new();
        assert!(t.apply(Event::Start, Some("ana"), 1));
        assert!(t.apply(Event::Resolve, None, 2));
        assert!(t.apply(Event::Close, None, 3));

        assert!(t.apply(Event::Reopen, None, 4));
        assert_eq!(t.status, Status::Open);
        assert_eq!(t.assignee, Some("ana"));
        assert_eq!(t.resolution, None);
        assert_eq!(t.closed_at, None);
    }

    #[test]
    fn start_from_hold_is_not_allowed_but_resume_is() {
        let mut t = Ticket::new();
        assert!(t.apply(Event::Start, Some("sam"), 1));
        assert!(t.apply(Event::Pause, None, 2));

        assert!(!t.apply(Event::Start, Some("pat"), 3));
        assert_eq!(t.status, Status::OnHold);
        assert_eq!(t.assignee, Some("sam"));

        assert!(t.apply(Event::Resume, Some("pat"), 4));
        assert_eq!(t.status, Status::InProgress);
        assert_eq!(t.assignee, Some("sam"));
    }

    #[test]
    fn cannot_close_without_resolution_and_reopen_allows_new_resolution() {
        let mut t = Ticket::new();
        assert!(!t.apply(Event::Close, None, 1));
        assert!(t.apply(Event::Start, Some("ivy"), 2));
        assert!(t.apply(Event::Resolve, None, 3));
        assert!(t.apply(Event::Close, None, 4));
        assert!(t.apply(Event::Reopen, None, 5));

        assert!(t.apply(Event::Start, None, 6));
        assert!(t.apply(Event::Resolve, None, 7));
        assert_eq!(t.status, Status::Resolved);
        assert_eq!(t.resolution, Some("fixed"));
        assert_eq!(t.closed_at, None);
    }
}
