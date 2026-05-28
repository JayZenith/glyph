#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Draft,
    Submitted,
    Approved,
    Rejected,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    Approve,
    Reject,
    Revise,
    Archive,
    Restore,
}

pub fn apply_event(status: Status, event: Event, has_reviewer: bool, has_changes_requested: bool) -> Status {
    match event {
        Event::Submit => Status::Submitted,
        Event::Approve => {
            if has_reviewer {
                Status::Approved
            } else {
                status
            }
        }
        Event::Reject => Status::Rejected,
        Event::Revise => Status::Draft,
        Event::Archive => Status::Archived,
        Event::Restore => {
            if has_changes_requested {
                Status::Draft
            } else {
                Status::Approved
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_event, Event, Status};

    #[test]
    fn submit_only_moves_draft_to_submitted() {
        assert_eq!(apply_event(Status::Draft, Event::Submit, false, false), Status::Submitted);
        assert_eq!(apply_event(Status::Rejected, Event::Submit, false, false), Status::Rejected);
        assert_eq!(apply_event(Status::Approved, Event::Submit, true, false), Status::Approved);
    }

    #[test]
    fn approve_requires_submitted_and_reviewer() {
        assert_eq!(apply_event(Status::Submitted, Event::Approve, true, false), Status::Approved);
        assert_eq!(apply_event(Status::Submitted, Event::Approve, false, false), Status::Submitted);
        assert_eq!(apply_event(Status::Draft, Event::Approve, true, false), Status::Draft);
    }

    #[test]
    fn reject_only_from_submitted() {
        assert_eq!(apply_event(Status::Submitted, Event::Reject, true, false), Status::Rejected);
        assert_eq!(apply_event(Status::Draft, Event::Reject, true, false), Status::Draft);
        assert_eq!(apply_event(Status::Approved, Event::Reject, true, false), Status::Approved);
    }

    #[test]
    fn revise_only_from_rejected_with_changes_requested() {
        assert_eq!(apply_event(Status::Rejected, Event::Revise, false, true), Status::Draft);
        assert_eq!(apply_event(Status::Rejected, Event::Revise, false, false), Status::Rejected);
        assert_eq!(apply_event(Status::Submitted, Event::Revise, false, true), Status::Submitted);
    }

    #[test]
    fn archive_only_from_terminal_review_states() {
        assert_eq!(apply_event(Status::Approved, Event::Archive, true, false), Status::Archived);
        assert_eq!(apply_event(Status::Rejected, Event::Archive, false, true), Status::Archived);
        assert_eq!(apply_event(Status::Draft, Event::Archive, false, false), Status::Draft);
        assert_eq!(apply_event(Status::Submitted, Event::Archive, true, false), Status::Submitted);
    }

    #[test]
    fn restore_from_archived_depends_on_pending_changes() {
        assert_eq!(apply_event(Status::Archived, Event::Restore, false, true), Status::Draft);
        assert_eq!(apply_event(Status::Archived, Event::Restore, false, false), Status::Approved);
        assert_eq!(apply_event(Status::Approved, Event::Restore, false, true), Status::Approved);
    }
}
