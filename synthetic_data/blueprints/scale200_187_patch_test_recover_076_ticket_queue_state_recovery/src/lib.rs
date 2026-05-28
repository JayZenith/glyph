#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    Active,
    Blocked,
    Done,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub touched: u32,
    pub blocked_count: u32,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::New,
            touched: 0,
            blocked_count: 0,
        }
    }

    pub fn apply(&mut self, event: &Event) {
        match (&self.status, event) {
            (Status::New, Event::Start) => {
                self.status = Status::Active;
                self.touched += 1;
            }
            (Status::Active, Event::Block) => {
                self.status = Status::Blocked;
                self.blocked_count += 1;
                self.touched += 1;
            }
            (Status::Blocked, Event::Unblock) => {
                self.status = Status::Active;
                self.touched += 1;
            }
            (_, Event::Complete) => {
                self.status = Status::Done;
                self.touched += 1;
            }
            (_, Event::Cancel) => {
                self.status = Status::Cancelled;
                self.touched += 1;
            }
            (_, Event::Reopen) => {
                self.status = Status::Active;
                self.touched += 1;
            }
            _ => {}
        }
    }
}

pub fn run(events: &[Event]) -> Ticket {
    let mut ticket = Ticket::new();
    for e in events {
        ticket.apply(e);
    }
    ticket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_complete_without_being_active() {
        let t = run(&[Event::Complete]);
        assert_eq!(t.status, Status::New);
        assert_eq!(t.touched, 0);
    }

    #[test]
    fn done_and_cancelled_require_reopen_before_work_resumes() {
        let done = run(&[Event::Start, Event::Complete, Event::Block, Event::Unblock]);
        assert_eq!(done.status, Status::Done);
        assert_eq!(done.touched, 2);

        let cancelled = run(&[Event::Start, Event::Cancel, Event::Start, Event::Complete]);
        assert_eq!(cancelled.status, Status::Cancelled);
        assert_eq!(cancelled.touched, 2);
    }

    #[test]
    fn reopen_goes_to_new_and_requires_start_again() {
        let t = run(&[Event::Start, Event::Complete, Event::Reopen, Event::Complete, Event::Start]);
        assert_eq!(t.status, Status::Active);
        assert_eq!(t.touched, 4);
    }

    #[test]
    fn blocked_cycle_counts_only_real_transitions() {
        let t = run(&[
            Event::Start,
            Event::Block,
            Event::Block,
            Event::Unblock,
            Event::Unblock,
            Event::Complete,
        ]);
        assert_eq!(t.status, Status::Done);
        assert_eq!(t.blocked_count, 1);
        assert_eq!(t.touched, 4);
    }
}
