#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    Assigned,
    InProgress,
    Paused,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub status: Status,
    pub assignee: Option<&'static str>,
    pub resolution: Option<&'static str>,
    pub reopen_count: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Open,
    Assign(&'static str),
    StartWork,
    Pause,
    Resume,
    Resolve(&'static str),
    Reopen,
    Close,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::New,
            assignee: None,
            resolution: None,
            reopen_count: 0,
        }
    }

    pub fn apply(&mut self, event: Event) {
        match event {
            Event::Open => {
                self.status = Status::New;
                self.assignee = None;
                self.resolution = None;
                self.reopen_count = 0;
            }
            Event::Assign(name) => {
                if self.status != Status::Closed {
                    self.assignee = Some(name);
                    if self.status == Status::New {
                        self.status = Status::Assigned;
                    }
                }
            }
            Event::StartWork => {
                if matches!(self.status, Status::Assigned | Status::Paused) {
                    self.status = Status::InProgress;
                }
            }
            Event::Pause => {
                if self.status == Status::InProgress {
                    self.status = Status::Paused;
                }
            }
            Event::Resume => {
                if self.status == Status::Paused {
                    self.status = Status::InProgress;
                }
            }
            Event::Resolve(note) => {
                if matches!(self.status, Status::InProgress | Status::Paused) {
                    self.status = Status::Resolved;
                    self.resolution = Some(note);
                }
            }
            Event::Reopen => {
                if matches!(self.status, Status::Resolved | Status::Closed) {
                    self.status = Status::Assigned;
                    self.reopen_count += 1;
                }
            }
            Event::Close => {
                if self.status == Status::Resolved {
                    self.status = Status::Closed;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolved_ticket_reopens_with_assignee_and_clears_resolution() {
        let mut t = Ticket::new();
        t.apply(Event::Assign("ops"));
        t.apply(Event::StartWork);
        t.apply(Event::Resolve("fixed config"));

        t.apply(Event::Reopen);

        assert_eq!(t.status, Status::Assigned);
        assert_eq!(t.assignee, Some("ops"));
        assert_eq!(t.resolution, None);
        assert_eq!(t.reopen_count, 1);
    }

    #[test]
    fn closed_ticket_can_reopen_without_losing_previous_assignee() {
        let mut t = Ticket::new();
        t.apply(Event::Assign("alice"));
        t.apply(Event::StartWork);
        t.apply(Event::Resolve("done"));
        t.apply(Event::Close);

        t.apply(Event::Reopen);

        assert_eq!(t.status, Status::Assigned);
        assert_eq!(t.assignee, Some("alice"));
        assert_eq!(t.resolution, None);
        assert_eq!(t.reopen_count, 1);
    }

    #[test]
    fn resolve_requires_assignee_and_pause_resume_flow_still_works() {
        let mut t = Ticket::new();
        t.apply(Event::StartWork);
        t.apply(Event::Resolve("should be ignored"));
        assert_eq!(t.status, Status::New);
        assert_eq!(t.resolution, None);

        t.apply(Event::Assign("bob"));
        t.apply(Event::StartWork);
        t.apply(Event::Pause);
        t.apply(Event::Resolve("investigated"));

        assert_eq!(t.status, Status::Resolved);
        assert_eq!(t.assignee, Some("bob"));
        assert_eq!(t.resolution, Some("investigated"));
    }

    #[test]
    fn assign_after_resolution_does_not_change_assignee_until_reopened() {
        let mut t = Ticket::new();
        t.apply(Event::Assign("primary"));
        t.apply(Event::StartWork);
        t.apply(Event::Resolve("complete"));

        t.apply(Event::Assign("secondary"));

        assert_eq!(t.status, Status::Resolved);
        assert_eq!(t.assignee, Some("primary"));
        assert_eq!(t.resolution, Some("complete"));
    }
}
