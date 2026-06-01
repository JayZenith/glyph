#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Guest,
    Member,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    View,
    Edit,
    Delete,
    BanUser,
    Publish { reviewed: bool },
    Export { include_private: bool },
}

pub fn permission(role: Role, action: Action, suspended: bool) -> &'static str {
    match action {
        Action::View => "allow",
        Action::Edit => match role {
            Role::Guest => "allow",
            Role::Member => "deny",
            Role::Admin => "allow",
        },
        Action::Delete => match role {
            Role::Admin => "review",
            _ => "deny",
        },
        Action::BanUser => match role {
            Role::Admin => "allow",
            _ => "deny",
        },
        Action::Publish { reviewed } => match role {
            Role::Admin => "allow",
            Role::Member if reviewed => "review",
            Role::Member => "allow",
            Role::Guest => "deny",
        },
        Action::Export { include_private } => {
            if include_private {
                match role {
                    Role::Admin => "allow",
                    Role::Member => "allow",
                    Role::Guest => "deny",
                }
            } else {
                "deny"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{permission, Action, Role};

    #[test]
    fn guest_permissions() {
        assert_eq!(permission(Role::Guest, Action::View, false), "allow");
        assert_eq!(permission(Role::Guest, Action::Edit, false), "deny");
        assert_eq!(permission(Role::Guest, Action::Publish { reviewed: true }, false), "deny");
    }

    #[test]
    fn member_permissions() {
        assert_eq!(permission(Role::Member, Action::Edit, false), "allow");
        assert_eq!(permission(Role::Member, Action::Delete, false), "review");
        assert_eq!(permission(Role::Member, Action::Publish { reviewed: false }, false), "review");
        assert_eq!(permission(Role::Member, Action::Publish { reviewed: true }, false), "allow");
    }

    #[test]
    fn export_rules() {
        assert_eq!(permission(Role::Member, Action::Export { include_private: false }, false), "allow");
        assert_eq!(permission(Role::Guest, Action::Export { include_private: false }, false), "allow");
        assert_eq!(permission(Role::Member, Action::Export { include_private: true }, false), "review");
        assert_eq!(permission(Role::Admin, Action::Export { include_private: true }, false), "allow");
    }

    #[test]
    fn suspension_overrides_mutating_actions_only() {
        assert_eq!(permission(Role::Admin, Action::View, true), "allow");
        assert_eq!(permission(Role::Admin, Action::Delete, true), "deny");
        assert_eq!(permission(Role::Member, Action::Publish { reviewed: true }, true), "deny");
        assert_eq!(permission(Role::Guest, Action::Export { include_private: false }, true), "allow");
    }

    #[test]
    fn admin_delete_requires_no_review() {
        assert_eq!(permission(Role::Admin, Action::Delete, false), "allow");
        assert_eq!(permission(Role::Admin, Action::BanUser, false), "allow");
    }
}
