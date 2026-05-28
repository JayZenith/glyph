#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Draft,
    Review,
    Approved,
    Rejected,
    Published,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Submit,
    Approve,
    Reject,
    Revise,
    Publish,
    Archive,
    Restore,
}

pub fn apply_events(events: &[Event]) -> Status {
    let mut status = Status::Draft;

    for event in events {
        status = match (status, *event) {
            (Status::Draft, Event::Submit) => Status::Review,
            (Status::Review, Event::Approve) => Status::Approved,
            (Status::Review, Event::Reject) => Status::Rejected,
            (Status::Rejected, Event::Revise) => Status::Draft,
            (Status::Approved, Event::Publish) => Status::Published,
            (Status::Published, Event::Archive) => Status::Archived,
            (Status::Archived, Event::Restore) => Status::Draft,
            (_, Event::Archive) => Status::Archived,
            (_, Event::Restore) => Status::Draft,
            _ => status,
        };
    }

    status
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event::*, Status::*};

    #[test]
    fn ignores_invalid_events_instead_of_jumping_states() {
        let status = apply_events(&[Publish, Submit, Approve, Publish]);
        assert_eq!(status, Published);
    }

    #[test]
    fn archive_only_works_from_published() {
        assert_eq!(apply_events(&[Archive]), Draft);
        assert_eq!(apply_events(&[Submit, Archive]), Review);
        assert_eq!(apply_events(&[Submit, Approve, Archive]), Approved);
        assert_eq!(apply_events(&[Submit, Approve, Publish, Archive]), Archived);
    }

    #[test]
    fn restore_returns_archived_items_to_draft_only() {
        assert_eq!(apply_events(&[Restore]), Draft);
        assert_eq!(apply_events(&[Submit, Approve, Restore]), Approved);
        assert_eq!(apply_events(&[Submit, Approve, Publish, Archive, Restore]), Draft);
    }

    #[test]
    fn rejected_items_can_be_reworked_through_full_flow() {
        let status = apply_events(&[Submit, Reject, Revise, Submit, Approve, Publish]);
        assert_eq!(status, Published);
    }

    #[test]
    fn terminal_state_can_be_reentered_after_restore() {
        let status = apply_events(&[
            Submit,
            Approve,
            Publish,
            Archive,
            Restore,
            Submit,
            Approve,
            Publish,
        ]);
        assert_eq!(status, Published);
    }
}
