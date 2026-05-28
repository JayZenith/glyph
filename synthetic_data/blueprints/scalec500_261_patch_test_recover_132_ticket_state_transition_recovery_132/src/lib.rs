#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    InProgress,
    Blocked,
    Done,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Start,
    Block,
    Unblock,
    Complete,
    Cancel,
    Reopen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub status: Status,
    pub assignee: Option<&'static str>,
    pub resolution: Option<&'static str>,
    pub block_reason: Option<&'static str>,
    pub history: Vec<Status>,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::New,
            assignee: None,
            resolution: None,
            block_reason: None,
            history: vec![Status::New],
        }
    }

    pub fn assign(&mut self, who: &'static str) {
        self.assignee = Some(who);
    }

    pub fn transition(&mut self, event: Event) -> bool {
        let next = match (self.status, event) {
            (Status::New, Event::Start) if self.assignee.is_some() => Some(Status::InProgress),
            (Status::InProgress, Event::Block) => Some(Status::Blocked),
            (Status::Blocked, Event::Unblock) => Some(Status::InProgress),
            (Status::InProgress, Event::Complete) => Some(Status::Done),
            (_, Event::Cancel) => Some(Status::Canceled),
            (Status::Done, Event::Reopen) => Some(Status::New),
            (Status::Canceled, Event::Reopen) => Some(Status::New),
            _ => None,
        };

        if let Some(next_status) = next {
            self.status = next_status;
            match event {
                Event::Block => {
                    self.block_reason = Some("waiting on dependency");
                }
                Event::Complete => {
                    self.resolution = Some("completed");
                }
                Event::Cancel => {
                    self.resolution = Some("canceled");
                }
                Event::Reopen => {
                    self.assignee = None;
                }
                _ => {}
            }
            self.history.push(self.status);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_requires_assignment() {
        let mut t = Ticket::new();
        assert!(!t.transition(Event::Start));
        t.assign("dev");
        assert!(t.transition(Event::Start));
        assert_eq!(t.status, Status::InProgress);
        assert_eq!(t.history, vec![Status::New, Status::InProgress]);
    }

    #[test]
    fn blocking_and_unblocking_manage_reason() {
        let mut t = Ticket::new();
        t.assign("dev");
        t.transition(Event::Start);
        assert!(t.transition(Event::Block));
        assert_eq!(t.status, Status::Blocked);
        assert_eq!(t.block_reason, Some("waiting on dependency"));

        assert!(t.transition(Event::Unblock));
        assert_eq!(t.status, Status::InProgress);
        assert_eq!(t.block_reason, None);
    }

    #[test]
    fn complete_sets_resolution_and_cancel_is_restricted() {
        let mut t = Ticket::new();
        assert!(t.transition(Event::Cancel));
        assert_eq!(t.status, Status::Canceled);
        assert_eq!(t.resolution, Some("canceled"));
        assert!(!t.transition(Event::Cancel));

        let mut t2 = Ticket::new();
        t2.assign("dev");
        t2.transition(Event::Start);
        assert!(t2.transition(Event::Complete));
        assert_eq!(t2.status, Status::Done);
        assert_eq!(t2.resolution, Some("completed"));
    }

    #[test]
    fn reopen_clears_terminal_metadata_but_keeps_assignee() {
        let mut t = Ticket::new();
        t.assign("dev");
        t.transition(Event::Start);
        t.transition(Event::Block);
        assert!(t.transition(Event::Cancel));
        assert_eq!(t.status, Status::Canceled);
        assert_eq!(t.block_reason, Some("waiting on dependency"));
        assert_eq!(t.resolution, Some("canceled"));

        assert!(t.transition(Event::Reopen));
        assert_eq!(t.status, Status::New);
        assert_eq!(t.assignee, Some("dev"));
        assert_eq!(t.resolution, None);
        assert_eq!(t.block_reason, None);
        assert_eq!(
            t.history,
            vec![
                Status::New,
                Status::InProgress,
                Status::Blocked,
                Status::Canceled,
                Status::New,
            ]
        );
    }
}
