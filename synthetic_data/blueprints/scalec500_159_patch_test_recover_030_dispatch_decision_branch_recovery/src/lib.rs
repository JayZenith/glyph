#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    User,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    View,
    Edit,
    Delete,
    Export,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Dashboard,
    Report { sensitive: bool },
    UserProfile { is_self: bool },
    AuditLog,
}

pub fn decide(role: Role, op: Operation, resource: Resource, feature_export: bool) -> &'static str {
    match (role, op, resource) {
        (_, Operation::View, Resource::Dashboard) => "allow",
        (Role::Guest, Operation::View, Resource::Report { sensitive: _ }) => "allow",
        (_, Operation::Export, Resource::Report { sensitive: _ }) if feature_export => "allow",
        (Role::Admin, Operation::Delete, Resource::AuditLog) => "allow",
        (_, Operation::Edit, Resource::UserProfile { is_self: _ }) => "allow",
        (Role::Admin, Operation::Edit, Resource::Report { sensitive: _ }) => "allow",
        _ => "deny",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guest_cannot_view_sensitive_report() {
        assert_eq!(
            decide(Role::Guest, Operation::View, Resource::Report { sensitive: true }, false),
            "deny"
        );
    }

    #[test]
    fn user_can_view_non_sensitive_report() {
        assert_eq!(
            decide(Role::User, Operation::View, Resource::Report { sensitive: false }, false),
            "allow"
        );
    }

    #[test]
    fn export_requires_feature_and_non_guest() {
        assert_eq!(
            decide(Role::User, Operation::Export, Resource::Report { sensitive: false }, false),
            "deny"
        );
        assert_eq!(
            decide(Role::User, Operation::Export, Resource::Report { sensitive: false }, true),
            "allow"
        );
        assert_eq!(
            decide(Role::Guest, Operation::Export, Resource::Report { sensitive: false }, true),
            "deny"
        );
    }

    #[test]
    fn only_self_profile_is_editable_for_non_admins() {
        assert_eq!(
            decide(Role::User, Operation::Edit, Resource::UserProfile { is_self: true }, false),
            "allow"
        );
        assert_eq!(
            decide(Role::User, Operation::Edit, Resource::UserProfile { is_self: false }, false),
            "deny"
        );
        assert_eq!(
            decide(Role::Guest, Operation::Edit, Resource::UserProfile { is_self: true }, false),
            "deny"
        );
        assert_eq!(
            decide(Role::Admin, Operation::Edit, Resource::UserProfile { is_self: false }, false),
            "allow"
        );
    }

    #[test]
    fn only_admin_can_edit_sensitive_reports() {
        assert_eq!(
            decide(Role::Admin, Operation::Edit, Resource::Report { sensitive: true }, false),
            "allow"
        );
        assert_eq!(
            decide(Role::User, Operation::Edit, Resource::Report { sensitive: true }, false),
            "deny"
        );
    }

    #[test]
    fn delete_audit_log_is_admin_only() {
        assert_eq!(
            decide(Role::Admin, Operation::Delete, Resource::AuditLog, false),
            "allow"
        );
        assert_eq!(
            decide(Role::User, Operation::Delete, Resource::AuditLog, false),
            "deny"
        );
    }
}
