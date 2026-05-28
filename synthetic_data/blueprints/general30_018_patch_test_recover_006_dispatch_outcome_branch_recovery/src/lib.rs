#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    Member,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    View,
    Edit,
    Delete,
    Publish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Allow,
    Deny,
    Review,
}

pub fn decide(role: Role, action: Action, locked: bool) -> Outcome {
    match action {
        Action::View => Outcome::Allow,
        Action::Edit => match role {
            Role::Guest => Outcome::Deny,
            Role::Member => Outcome::Allow,
            Role::Admin => Outcome::Allow,
        },
        Action::Delete => match role {
            Role::Admin => Outcome::Allow,
            _ => Outcome::Deny,
        },
        Action::Publish => match role {
            Role::Guest => Outcome::Deny,
            Role::Member => Outcome::Allow,
            Role::Admin => Outcome::Allow,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guest_can_only_view() {
        assert_eq!(decide(Role::Guest, Action::View, false), Outcome::Allow);
        assert_eq!(decide(Role::Guest, Action::Edit, false), Outcome::Deny);
        assert_eq!(decide(Role::Guest, Action::Delete, false), Outcome::Deny);
        assert_eq!(decide(Role::Guest, Action::Publish, false), Outcome::Deny);
    }

    #[test]
    fn members_cannot_edit_locked_items() {
        assert_eq!(decide(Role::Member, Action::Edit, false), Outcome::Allow);
        assert_eq!(decide(Role::Member, Action::Edit, true), Outcome::Review);
    }

    #[test]
    fn deleting_locked_items_requires_review_even_for_admin() {
        assert_eq!(decide(Role::Admin, Action::Delete, false), Outcome::Allow);
        assert_eq!(decide(Role::Admin, Action::Delete, true), Outcome::Review);
    }

    #[test]
    fn publishing_requires_admin_but_locked_publish_is_never_allowed() {
        assert_eq!(decide(Role::Member, Action::Publish, false), Outcome::Review);
        assert_eq!(decide(Role::Admin, Action::Publish, false), Outcome::Allow);
        assert_eq!(decide(Role::Admin, Action::Publish, true), Outcome::Deny);
    }
}
