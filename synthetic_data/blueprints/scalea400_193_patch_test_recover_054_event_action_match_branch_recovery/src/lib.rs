#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    User,
    Moderator,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Dashboard,
    Project,
    Billing,
    AuditLog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    View,
    Edit,
    Delete,
}

pub fn is_allowed(role: Role, resource: Resource, action: Action, owner: bool) -> bool {
    match resource {
        Resource::Dashboard => match action {
            Action::View => role != Role::Guest,
            Action::Edit => matches!(role, Role::User | Role::Moderator | Role::Admin),
            Action::Delete => false,
        },
        Resource::Project => match action {
            Action::View => role != Role::Guest,
            Action::Edit => matches!(role, Role::Moderator | Role::Admin) || owner,
            Action::Delete => matches!(role, Role::Moderator | Role::Admin) || owner,
        },
        Resource::Billing => match action {
            Action::View => matches!(role, Role::Admin),
            Action::Edit => matches!(role, Role::Admin),
            Action::Delete => false,
        },
        Resource::AuditLog => match action {
            Action::View => matches!(role, Role::Moderator | Role::Admin),
            Action::Edit => matches!(role, Role::Moderator | Role::Admin),
            Action::Delete => matches!(role, Role::Admin),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dashboard_access_rules() {
        assert!(!is_allowed(Role::Guest, Resource::Dashboard, Action::View, false));
        assert!(is_allowed(Role::User, Resource::Dashboard, Action::View, false));
        assert!(is_allowed(Role::User, Resource::Dashboard, Action::Edit, false));
        assert!(!is_allowed(Role::Moderator, Resource::Dashboard, Action::Delete, false));
    }

    #[test]
    fn project_rules_distinguish_owner_from_role() {
        assert!(!is_allowed(Role::Guest, Resource::Project, Action::View, false));
        assert!(is_allowed(Role::User, Resource::Project, Action::View, true));
        assert!(is_allowed(Role::User, Resource::Project, Action::Edit, true));
        assert!(!is_allowed(Role::User, Resource::Project, Action::Edit, false));
        assert!(!is_allowed(Role::User, Resource::Project, Action::Delete, true));
        assert!(is_allowed(Role::Moderator, Resource::Project, Action::Delete, false));
    }

    #[test]
    fn billing_is_admin_only_and_never_deletable() {
        assert!(!is_allowed(Role::Moderator, Resource::Billing, Action::View, false));
        assert!(is_allowed(Role::Admin, Resource::Billing, Action::View, false));
        assert!(is_allowed(Role::Admin, Resource::Billing, Action::Edit, false));
        assert!(!is_allowed(Role::Admin, Resource::Billing, Action::Delete, false));
    }

    #[test]
    fn audit_log_is_view_only_for_moderator_and_admin_delete_only_for_admin() {
        assert!(is_allowed(Role::Moderator, Resource::AuditLog, Action::View, false));
        assert!(!is_allowed(Role::Moderator, Resource::AuditLog, Action::Edit, false));
        assert!(!is_allowed(Role::Moderator, Resource::AuditLog, Action::Delete, false));
        assert!(is_allowed(Role::Admin, Resource::AuditLog, Action::View, false));
        assert!(!is_allowed(Role::Admin, Resource::AuditLog, Action::Edit, false));
        assert!(is_allowed(Role::Admin, Resource::AuditLog, Action::Delete, false));
    }
}
