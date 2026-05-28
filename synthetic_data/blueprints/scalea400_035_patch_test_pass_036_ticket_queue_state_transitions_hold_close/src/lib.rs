#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    New,
    Queued,
    Assigned,
    OnHold,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub status: Status,
    pub assignee: Option<String>,
    pub hold_reason: Option<String>,
    pub resolution: Option<String>,
    pub closed: bool,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::New,
            assignee: None,
            hold_reason: None,
            resolution: None,
            closed: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Queue,
    Assign(&'static str),
    Hold(&'static str),
    Resume,
    Resolve(&'static str),
    Close,
    Reopen,
}

pub fn apply_events(events: &[Event]) -> Ticket {
    let mut ticket = Ticket::new();

    for event in events {
        match *event {
            Event::Queue => {
                if !ticket.closed && matches!(ticket.status, Status::New) {
                    ticket.status = Status::Queued;
                }
            }
            Event::Assign(name) => {
                if !ticket.closed && matches!(ticket.status, Status::New | Status::Queued) {
                    ticket.assignee = Some(name.to_string());
                    ticket.status = Status::Assigned;
                    ticket.hold_reason = None;
                }
            }
            Event::Hold(reason) => {
                if !ticket.closed && matches!(ticket.status, Status::Assigned | Status::Queued) {
                    ticket.status = Status::OnHold;
                    ticket.hold_reason = Some(reason.to_string());
                }
            }
            Event::Resume => {
                if !ticket.closed && matches!(ticket.status, Status::OnHold) {
                    ticket.status = Status::Assigned;
                }
            }
            Event::Resolve(note) => {
                if !ticket.closed && matches!(ticket.status, Status::Assigned | Status::OnHold) {
                    ticket.status = Status::Resolved;
                    ticket.resolution = Some(note.to_string());
                    ticket.hold_reason = None;
                }
            }
            Event::Close => {
                if matches!(ticket.status, Status::Resolved) {
                    ticket.status = Status::Closed;
                    ticket.closed = false;
                }
            }
            Event::Reopen => {
                if matches!(ticket.status, Status::Closed) {
                    ticket.status = Status::Queued;
                    ticket.assignee = None;
                }
            }
        }
    }

    ticket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hold_resume_close_reopen_obeys_invariants() {
        let ticket = apply_events(&[
            Event::Queue,
            Event::Assign("maya"),
            Event::Hold("waiting for customer"),
            Event::Resume,
            Event::Resolve("fixed in patch"),
            Event::Close,
            Event::Reopen,
        ]);

        assert_eq!(ticket.status, Status::Queued);
        assert_eq!(ticket.assignee, None);
        assert_eq!(ticket.hold_reason, None);
        assert_eq!(ticket.resolution, None);
        assert!(!ticket.closed);
    }

    #[test]
    fn close_makes_future_events_ignored_until_reopen_and_reopen_clears_resolution() {
        let ticket = apply_events(&[
            Event::Queue,
            Event::Assign("ivy"),
            Event::Resolve("done"),
            Event::Close,
            Event::Assign("noah"),
            Event::Hold("should not happen"),
            Event::Reopen,
            Event::Assign("zoe"),
        ]);

        assert_eq!(ticket.status, Status::Assigned);
        assert_eq!(ticket.assignee, Some("zoe".to_string()));
        assert_eq!(ticket.hold_reason, None);
        assert_eq!(ticket.resolution, None);
        assert!(!ticket.closed);
    }

    #[test]
    fn resume_from_hold_without_assignee_returns_to_queue() {
        let ticket = apply_events(&[
            Event::Queue,
            Event::Hold("capacity"),
            Event::Resume,
        ]);

        assert_eq!(ticket.status, Status::Queued);
        assert_eq!(ticket.assignee, None);
        assert_eq!(ticket.hold_reason, None);
        assert_eq!(ticket.resolution, None);
    }
}
