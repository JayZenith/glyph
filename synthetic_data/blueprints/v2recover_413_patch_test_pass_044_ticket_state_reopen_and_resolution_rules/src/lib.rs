#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    New,
    InProgress,
    Blocked,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    StartWork,
    Block,
    Unblock,
    Resolve,
    Close,
    Reopen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub status: Status,
    pub resolution: Option<&'static str>,
    pub reopen_count: u8,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::New,
            resolution: None,
            reopen_count: 0,
        }
    }

    pub fn apply(&mut self, event: Event) {
        match event {
            Event::StartWork => {
                if matches!(self.status, Status::New) {
                    self.status = Status::InProgress;
                }
            }
            Event::Block => {
                if matches!(self.status, Status::InProgress) {
                    self.status = Status::Blocked;
                }
            }
            Event::Unblock => {
                if matches!(self.status, Status::Blocked) {
                    self.status = Status::InProgress;
                }
            }
            Event::Resolve => {
                if matches!(self.status, Status::InProgress | Status::Blocked) {
                    self.status = Status::Resolved;
                    self.resolution = Some("done");
                }
            }
            Event::Close => {
                if matches!(self.status, Status::Resolved) {
                    self.status = Status::Closed;
                }
            }
            Event::Reopen => {
                if matches!(self.status, Status::Resolved | Status::Closed) {
                    self.status = Status::InProgress;
                }
            }
        }
    }
}

pub fn apply_events(events: &[Event]) -> Ticket {
    let mut ticket = Ticket::new();
    for event in events.iter().cloned() {
        ticket.apply(event);
    }
    ticket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reopen_from_resolved_clears_resolution_and_tracks_count() {
        let ticket = apply_events(&[
            Event::StartWork,
            Event::Resolve,
            Event::Reopen,
        ]);

        assert_eq!(ticket.status, Status::InProgress);
        assert_eq!(ticket.resolution, None);
        assert_eq!(ticket.reopen_count, 1);
    }

    #[test]
    fn reopen_from_closed_goes_to_in_progress_and_clears_resolution() {
        let ticket = apply_events(&[
            Event::StartWork,
            Event::Resolve,
            Event::Close,
            Event::Reopen,
        ]);

        assert_eq!(ticket.status, Status::InProgress);
        assert_eq!(ticket.resolution, None);
        assert_eq!(ticket.reopen_count, 1);
    }

    #[test]
    fn resolving_while_blocked_keeps_blocked_until_unblocked() {
        let ticket = apply_events(&[
            Event::StartWork,
            Event::Block,
            Event::Resolve,
        ]);

        assert_eq!(ticket.status, Status::Blocked);
        assert_eq!(ticket.resolution, None);
        assert_eq!(ticket.reopen_count, 0);
    }

    #[test]
    fn normal_resolution_and_close_still_work() {
        let ticket = apply_events(&[
            Event::StartWork,
            Event::Resolve,
            Event::Close,
        ]);

        assert_eq!(ticket.status, Status::Closed);
        assert_eq!(ticket.resolution, Some("done"));
        assert_eq!(ticket.reopen_count, 0);
    }

    #[test]
    fn repeated_reopens_accumulate_count() {
        let ticket = apply_events(&[
            Event::StartWork,
            Event::Resolve,
            Event::Reopen,
            Event::Resolve,
            Event::Close,
            Event::Reopen,
        ]);

        assert_eq!(ticket.status, Status::InProgress);
        assert_eq!(ticket.resolution, None);
        assert_eq!(ticket.reopen_count, 2);
    }
}
