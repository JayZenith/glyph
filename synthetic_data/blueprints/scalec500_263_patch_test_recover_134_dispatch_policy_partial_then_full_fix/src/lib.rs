#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    Member,
    Staff,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Dashboard,
    Billing,
    AuditLog,
    BetaLab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    View,
    Edit,
    Delete,
}

pub fn is_allowed(role: Role, resource: Resource, action: Action, beta_enabled: bool) -> bool {
    match resource {
        Resource::Dashboard => match action {
            Action::View => !matches!(role, Role::Guest),
            Action::Edit => matches!(role, Role::Staff | Role::Admin),
            Action::Delete => matches!(role, Role::Admin),
        },
        Resource::Billing => match action {
            Action::View => matches!(role, Role::Member | Role::Staff | Role::Admin),
            Action::Edit => matches!(role, Role::Admin),
            Action::Delete => matches!(role, Role::Admin),
        },
        Resource::AuditLog => match action {
            Action::View => matches!(role, Role::Staff | Role::Admin),
            Action::Edit => false,
            Action::Delete => false,
        },
        Resource::BetaLab => {
            if !beta_enabled {
                return false;
            }
            match action {
                Action::View => matches!(role, Role::Member | Role::Staff | Role::Admin),
                Action::Edit => matches!(role, Role::Staff),
                Action::Delete => matches!(role, Role::Admin),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dashboard_policy() {
        assert!(!is_allowed(Role::Guest, Resource::Dashboard, Action::View, true));
        assert!(is_allowed(Role::Member, Resource::Dashboard, Action::View, true));
        assert!(!is_allowed(Role::Member, Resource::Dashboard, Action::Edit, true));
        assert!(is_allowed(Role::Staff, Resource::Dashboard, Action::Edit, true));
        assert!(is_allowed(Role::Admin, Resource::Dashboard, Action::Delete, true));
    }

    #[test]
    fn billing_policy() {
        assert!(is_allowed(Role::Member, Resource::Billing, Action::View, true));
        assert!(is_allowed(Role::Staff, Resource::Billing, Action::Edit, true));
        assert!(!is_allowed(Role::Member, Resource::Billing, Action::Edit, true));
        assert!(is_allowed(Role::Admin, Resource::Billing, Action::Delete, true));
    }

    #[test]
    fn audit_log_policy() {
        assert!(!is_allowed(Role::Member, Resource::AuditLog, Action::View, true));
        assert!(is_allowed(Role::Staff, Resource::AuditLog, Action::View, true));
        assert!(!is_allowed(Role::Admin, Resource::AuditLog, Action::Delete, true));
    }

    #[test]
    fn beta_lab_policy_requires_flag_and_right_roles() {
        assert!(!is_allowed(Role::Admin, Resource::BetaLab, Action::View, false));
        assert!(!is_allowed(Role::Guest, Resource::BetaLab, Action::View, true));
        assert!(is_allowed(Role::Member, Resource::BetaLab, Action::View, true));
        assert!(is_allowed(Role::Admin, Resource::BetaLab, Action::Edit, true));
        assert!(!is_allowed(Role::Staff, Resource::BetaLab, Action::Delete, true));
        assert!(is_allowed(Role::Admin, Resource::BetaLab, Action::Delete, true));
    }
}
