#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TicketState {
    New,
    InProgress { blocked: bool },
    Resolved { verified: bool },
    Closed,
}

pub fn status_label(state: TicketState) -> &'static str {
    match state {
        TicketState::New => "new",
        TicketState::InProgress { blocked: true } => "blocked",
        TicketState::InProgress { blocked: false } => "working",
        TicketState::Resolved { verified: true } => "done",
        TicketState::Resolved { verified: false } => "pending-check",
        TicketState::Closed => "done",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_new_and_progress_states() {
        assert_eq!(status_label(TicketState::New), "new");
        assert_eq!(
            status_label(TicketState::InProgress { blocked: false }),
            "working"
        );
        assert_eq!(
            status_label(TicketState::InProgress { blocked: true }),
            "blocked"
        );
    }

    #[test]
    fn labels_resolved_variants_differ() {
        assert_eq!(
            status_label(TicketState::Resolved { verified: false }),
            "pending-check"
        );
        assert_eq!(
            status_label(TicketState::Resolved { verified: true }),
            "done"
        );
    }

    #[test]
    fn closed_is_distinct_from_resolved_done() {
        assert_eq!(status_label(TicketState::Closed), "archived");
    }
}
