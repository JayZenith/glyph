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
    Resume,
    Complete,
    Cancel,
    Reopen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub status: Status,
    pub assignee: Option<&'static str>,
    pub completed_steps: u32,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::New,
            assignee: None,
            completed_steps: 0,
        }
    }

    pub fn apply(&mut self, event: Event) {
        match event {
            Event::Start => {
                if self.status == Status::New || self.status == Status::Blocked {
                    self.status = Status::InProgress;
                    if self.assignee.is_none() {
                        self.assignee = Some("system");
                    }
                }
            }
            Event::Block => {
                if self.status == Status::InProgress {
                    self.status = Status::Blocked;
                }
            }
            Event::Resume => {
                if self.status == Status::Blocked {
                    self.status = Status::InProgress;
                }
            }
            Event::Complete => {
                if self.status == Status::InProgress || self.status == Status::Blocked {
                    self.status = Status::Done;
                    self.completed_steps += 1;
                }
            }
            Event::Cancel => {
                self.status = Status::Canceled;
                self.assignee = None;
            }
            Event::Reopen => {
                if self.status == Status::Done || self.status == Status::Canceled {
                    self.status = Status::New;
                }
            }
        }
    }
}

pub fn run_events(events: &[Event]) -> Ticket {
    let mut ticket = Ticket::new();
    for &event in events {
        ticket.apply(event);
    }
    ticket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocked_ticket_cannot_complete_without_resume() {
        let ticket = run_events(&[Event::Start, Event::Block, Event::Complete]);
        assert_eq!(ticket.status, Status::Blocked);
        assert_eq!(ticket.completed_steps, 0);
        assert_eq!(ticket.assignee, Some("system"));
    }

    #[test]
    fn cancel_is_terminal_until_reopen() {
        let ticket = run_events(&[Event::Start, Event::Cancel, Event::Start, Event::Complete]);
        assert_eq!(ticket.status, Status::Canceled);
        assert_eq!(ticket.completed_steps, 0);
        assert_eq!(ticket.assignee, None);
    }

    #[test]
    fn reopen_from_done_resets_progress_and_assignment() {
        let ticket = run_events(&[
            Event::Start,
            Event::Complete,
            Event::Reopen,
            Event::Start,
        ]);
        assert_eq!(ticket.status, Status::InProgress);
        assert_eq!(ticket.completed_steps, 0);
        assert_eq!(ticket.assignee, Some("system"));
    }

    #[test]
    fn resume_only_works_from_blocked() {
        let ticket = run_events(&[Event::Resume, Event::Start, Event::Resume]);
        assert_eq!(ticket.status, Status::InProgress);
        assert_eq!(ticket.completed_steps, 0);
    }
}
