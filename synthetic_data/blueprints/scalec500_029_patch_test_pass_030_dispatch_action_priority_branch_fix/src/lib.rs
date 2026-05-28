#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    User,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Read,
    Write,
    Delete,
    Audit,
}

pub fn permission_label(role: Role, action: Action) -> &'static str {
    match action {
        Action::Read => "allow",
        Action::Write => match role {
            Role::Guest => "deny",
            _ => "allow",
        },
        Action::Delete => match role {
            Role::Admin => "allow",
            _ => "deny",
        },
        Action::Audit => match role {
            Role::Guest => "deny",
            Role::User => "allow",
            Role::Admin => "review",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_is_allowed_for_everyone() {
        assert_eq!(permission_label(Role::Guest, Action::Read), "allow");
        assert_eq!(permission_label(Role::User, Action::Read), "allow");
        assert_eq!(permission_label(Role::Admin, Action::Read), "allow");
    }

    #[test]
    fn write_requires_non_guest() {
        assert_eq!(permission_label(Role::Guest, Action::Write), "deny");
        assert_eq!(permission_label(Role::User, Action::Write), "allow");
        assert_eq!(permission_label(Role::Admin, Action::Write), "allow");
    }

    #[test]
    fn delete_requires_admin() {
        assert_eq!(permission_label(Role::Guest, Action::Delete), "deny");
        assert_eq!(permission_label(Role::User, Action::Delete), "deny");
        assert_eq!(permission_label(Role::Admin, Action::Delete), "allow");
    }

    #[test]
    fn audit_requires_review_for_regular_users() {
        assert_eq!(permission_label(Role::Guest, Action::Audit), "deny");
        assert_eq!(permission_label(Role::User, Action::Audit), "review");
        assert_eq!(permission_label(Role::Admin, Action::Audit), "allow");
    }
}
