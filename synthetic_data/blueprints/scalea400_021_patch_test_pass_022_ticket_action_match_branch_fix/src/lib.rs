#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TicketState {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    Reporter,
    Agent,
    Manager,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Comment,
    StartWork,
    Resolve,
    Reopen,
    Close,
}

pub fn allowed(state: TicketState, role: UserRole, action: Action) -> bool {
    match action {
        Action::Comment => true,
        Action::StartWork => matches!(state, TicketState::Open) && matches!(role, UserRole::Agent),
        Action::Resolve => matches!(state, TicketState::InProgress) && matches!(role, UserRole::Agent),
        Action::Reopen => matches!(state, TicketState::Resolved | TicketState::Closed),
        Action::Close => {
            matches!(state, TicketState::Resolved)
                && matches!(role, UserRole::Agent | UserRole::Manager)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reporter_cannot_reopen_closed_ticket() {
        assert!(!allowed(
            TicketState::Closed,
            UserRole::Reporter,
            Action::Reopen
        ));
    }

    #[test]
    fn agent_can_reopen_resolved_ticket() {
        assert!(allowed(
            TicketState::Resolved,
            UserRole::Agent,
            Action::Reopen
        ));
    }

    #[test]
    fn manager_can_reopen_closed_ticket() {
        assert!(allowed(
            TicketState::Closed,
            UserRole::Manager,
            Action::Reopen
        ));
    }

    #[test]
    fn reopen_not_allowed_from_open_state() {
        assert!(!allowed(
            TicketState::Open,
            UserRole::Manager,
            Action::Reopen
        ));
    }

    #[test]
    fn close_requires_resolved_state() {
        assert!(!allowed(
            TicketState::Closed,
            UserRole::Manager,
            Action::Close
        ));
    }
}
