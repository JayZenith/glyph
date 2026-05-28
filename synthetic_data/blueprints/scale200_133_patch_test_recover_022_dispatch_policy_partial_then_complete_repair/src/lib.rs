#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Create,
    Update,
    Delete,
    Archive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    Member,
    Moderator,
    Admin,
}

pub fn can_execute(role: Role, action: Action, locked: bool) -> bool {
    match action {
        Action::Create => match role {
            Role::Guest => false,
            Role::Member | Role::Moderator | Role::Admin => true,
        },
        Action::Update => match role {
            Role::Guest => false,
            Role::Member => !locked,
            Role::Moderator | Role::Admin => true,
        },
        Action::Delete => match role {
            Role::Admin => true,
            Role::Moderator => true,
            _ => false,
        },
        Action::Archive => match role {
            Role::Admin => true,
            Role::Moderator => false,
            Role::Member => true,
            Role::Guest => false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_permissions() {
        assert!(!can_execute(Role::Guest, Action::Create, false));
        assert!(can_execute(Role::Member, Action::Create, false));
        assert!(can_execute(Role::Moderator, Action::Create, true));
        assert!(can_execute(Role::Admin, Action::Create, true));
    }

    #[test]
    fn update_respects_locked_state() {
        assert!(!can_execute(Role::Guest, Action::Update, false));
        assert!(can_execute(Role::Member, Action::Update, false));
        assert!(!can_execute(Role::Member, Action::Update, true));
        assert!(can_execute(Role::Moderator, Action::Update, true));
        assert!(can_execute(Role::Admin, Action::Update, true));
    }

    #[test]
    fn delete_requires_unlocked_and_elevated_role() {
        assert!(!can_execute(Role::Guest, Action::Delete, false));
        assert!(!can_execute(Role::Member, Action::Delete, false));
        assert!(can_execute(Role::Moderator, Action::Delete, false));
        assert!(!can_execute(Role::Moderator, Action::Delete, true));
        assert!(can_execute(Role::Admin, Action::Delete, false));
        assert!(!can_execute(Role::Admin, Action::Delete, true));
    }

    #[test]
    fn archive_is_admin_only_and_requires_locked() {
        assert!(!can_execute(Role::Guest, Action::Archive, false));
        assert!(!can_execute(Role::Member, Action::Archive, true));
        assert!(!can_execute(Role::Moderator, Action::Archive, true));
        assert!(!can_execute(Role::Admin, Action::Archive, false));
        assert!(can_execute(Role::Admin, Action::Archive, true));
    }
}
