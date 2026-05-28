#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TicketState {
    New,
    Assigned,
    Escalated,
    Resolved,
    Closed,
}

pub fn action_label(state: TicketState, urgent: bool) -> &'static str {
    match state {
        TicketState::New | TicketState::Assigned => {
            if urgent { "queue" } else { "work" }
        }
        TicketState::Escalated => {
            if urgent { "queue" } else { "review" }
        }
        TicketState::Resolved | TicketState::Closed => "done",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_assigned_use_normal_flow_when_not_urgent() {
        assert_eq!(action_label(TicketState::New, false), "work");
        assert_eq!(action_label(TicketState::Assigned, false), "work");
    }

    #[test]
    fn urgent_new_and_assigned_are_expedited() {
        assert_eq!(action_label(TicketState::New, true), "queue");
        assert_eq!(action_label(TicketState::Assigned, true), "queue");
    }

    #[test]
    fn escalated_always_goes_to_review() {
        assert_eq!(action_label(TicketState::Escalated, false), "review");
        assert_eq!(action_label(TicketState::Escalated, true), "review");
    }

    #[test]
    fn finished_states_are_done() {
        assert_eq!(action_label(TicketState::Resolved, false), "done");
        assert_eq!(action_label(TicketState::Closed, true), "done");
    }
}
