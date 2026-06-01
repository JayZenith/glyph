#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Draft,
    InReview,
    Approved,
    Published,
    Rejected,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub status: Status,
    pub revisions: u32,
    pub review_rounds: u32,
    pub approvals: u32,
    pub published_version: Option<u32>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            status: Status::Draft,
            revisions: 0,
            review_rounds: 0,
            approvals: 0,
            published_version: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Edit,
    Submit,
    Approve,
    Reject,
    Publish,
    Archive,
    Restore,
}

pub fn apply_event(doc: &mut Document, event: Event) {
    match event {
        Event::Edit => {
            doc.revisions += 1;
            doc.status = Status::Draft;
            doc.approvals = 0;
        }
        Event::Submit => {
            doc.status = Status::InReview;
        }
        Event::Approve => {
            doc.approvals += 1;
            doc.status = Status::Approved;
        }
        Event::Reject => {
            doc.status = Status::Rejected;
        }
        Event::Publish => {
            doc.status = Status::Published;
            doc.published_version = Some(doc.revisions);
        }
        Event::Archive => {
            doc.status = Status::Archived;
        }
        Event::Restore => {
            doc.status = Status::Draft;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draft_submit_approve_publish_records_review_round_and_version() {
        let mut doc = Document::new();
        apply_event(&mut doc, Event::Edit);
        apply_event(&mut doc, Event::Submit);
        apply_event(&mut doc, Event::Approve);
        apply_event(&mut doc, Event::Publish);

        assert_eq!(doc.status, Status::Published);
        assert_eq!(doc.revisions, 1);
        assert_eq!(doc.review_rounds, 1);
        assert_eq!(doc.approvals, 1);
        assert_eq!(doc.published_version, Some(1));
    }

    #[test]
    fn edit_during_review_returns_to_draft_and_counts_revision() {
        let mut doc = Document::new();
        apply_event(&mut doc, Event::Submit);
        apply_event(&mut doc, Event::Edit);

        assert_eq!(doc.status, Status::Draft);
        assert_eq!(doc.revisions, 1);
        assert_eq!(doc.review_rounds, 1);
        assert_eq!(doc.approvals, 0);
    }

    #[test]
    fn approval_requires_review_state() {
        let mut doc = Document::new();
        apply_event(&mut doc, Event::Approve);

        assert_eq!(doc.status, Status::Draft);
        assert_eq!(doc.approvals, 0);
    }

    #[test]
    fn publish_requires_approved_state() {
        let mut doc = Document::new();
        apply_event(&mut doc, Event::Edit);
        apply_event(&mut doc, Event::Publish);

        assert_eq!(doc.status, Status::Draft);
        assert_eq!(doc.published_version, None);
    }

    #[test]
    fn reject_only_from_review_and_restore_goes_to_draft() {
        let mut doc = Document::new();
        apply_event(&mut doc, Event::Reject);
        assert_eq!(doc.status, Status::Draft);

        apply_event(&mut doc, Event::Submit);
        apply_event(&mut doc, Event::Reject);
        assert_eq!(doc.status, Status::Rejected);

        apply_event(&mut doc, Event::Restore);
        assert_eq!(doc.status, Status::Draft);
        assert_eq!(doc.approvals, 0);
    }

    #[test]
    fn archive_only_from_published_and_restore_preserves_published_version() {
        let mut doc = Document::new();
        apply_event(&mut doc, Event::Edit);
        apply_event(&mut doc, Event::Submit);
        apply_event(&mut doc, Event::Approve);
        apply_event(&mut doc, Event::Publish);
        apply_event(&mut doc, Event::Archive);

        assert_eq!(doc.status, Status::Archived);
        assert_eq!(doc.published_version, Some(1));

        apply_event(&mut doc, Event::Restore);
        assert_eq!(doc.status, Status::Published);
        assert_eq!(doc.published_version, Some(1));
    }

    #[test]
    fn resubmission_after_rejection_starts_new_review_round() {
        let mut doc = Document::new();
        apply_event(&mut doc, Event::Submit);
        apply_event(&mut doc, Event::Reject);
        apply_event(&mut doc, Event::Restore);
        apply_event(&mut doc, Event::Edit);
        apply_event(&mut doc, Event::Submit);

        assert_eq!(doc.status, Status::InReview);
        assert_eq!(doc.review_rounds, 2);
        assert_eq!(doc.revisions, 1);
    }
}
