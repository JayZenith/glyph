#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Draft,
    Submitted,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    Approve,
    Reject,
    Reopen,
}

pub fn apply_event(status: Status, event: Event) -> Status {
    match (status, event) {
        (Status::Draft, Event::Submit) => Status::Submitted,
        (Status::Submitted, Event::Approve) => Status::Approved,
        (Status::Submitted, Event::Reject) => Status::Rejected,
        (Status::Rejected, Event::Reopen) => Status::Submitted,
        _ => status,
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_event, Event, Status};

    #[test]
    fn submit_and_approve_flow() {
        let s = apply_event(Status::Draft, Event::Submit);
        assert_eq!(s, Status::Submitted);
        let s = apply_event(s, Event::Approve);
        assert_eq!(s, Status::Approved);
    }

    #[test]
    fn approved_order_can_be_reopened_to_draft() {
        let s = apply_event(Status::Approved, Event::Reopen);
        assert_eq!(s, Status::Draft);
    }

    #[test]
    fn rejected_order_can_be_reopened_to_draft() {
        let s = apply_event(Status::Rejected, Event::Reopen);
        assert_eq!(s, Status::Draft);
    }

    #[test]
    fn invalid_events_leave_state_unchanged() {
        assert_eq!(apply_event(Status::Draft, Event::Approve), Status::Draft);
        assert_eq!(apply_event(Status::Approved, Event::Submit), Status::Approved);
    }
}
