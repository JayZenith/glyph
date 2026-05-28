#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Draft,
    InReview,
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

pub fn apply_events(initial: Status, events: &[Event]) -> Status {
    let mut status = initial;
    for &event in events {
        status = step(status, event);
    }
    status
}

fn step(status: Status, event: Event) -> Status {
    use Event::*;
    use Status::*;

    match (status, event) {
        (Draft, Submit) => InReview,
        (InReview, Approve) => Approved,
        (InReview, Reject) => Rejected,
        (Rejected, Revise) => Draft,
        (Approved, Publish) => Published,
        (Published, Archive) => Archived,
        (Archived, Restore) => Draft,
        _ => status,
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_events, Event::*, Status::*};

    #[test]
    fn full_happy_path_reaches_published() {
        let got = apply_events(Draft, &[Submit, Approve, Publish]);
        assert_eq!(got, Published);
    }

    #[test]
    fn rejection_can_be_revised_and_resubmitted() {
        let got = apply_events(Draft, &[Submit, Reject, Revise, Submit, Approve]);
        assert_eq!(got, Approved);
    }

    #[test]
    fn archive_restore_keeps_published_items_published() {
        let got = apply_events(Published, &[Archive, Restore]);
        assert_eq!(got, Published);
    }

    #[test]
    fn reject_after_approval_sends_back_to_rejected() {
        let got = apply_events(Draft, &[Submit, Approve, Reject]);
        assert_eq!(got, Rejected);
    }

    #[test]
    fn invalid_events_do_not_change_state() {
        let got = apply_events(Draft, &[Publish, Archive, Restore]);
        assert_eq!(got, Draft);
    }
}
