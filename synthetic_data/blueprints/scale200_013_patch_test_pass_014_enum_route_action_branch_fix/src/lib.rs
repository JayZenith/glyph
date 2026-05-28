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

pub fn is_allowed(role: Role, action: Action, archived: bool) -> bool {
    match action {
        Action::View => !archived,
        Action::Edit => matches!(role, Role::Member | Role::Admin) && !archived,
        Action::Delete => role == Role::Admin || archived,
        Action::Publish => role == Role::Member || role == Role::Admin,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guest_can_only_view_active_items() {
        assert!(is_allowed(Role::Guest, Action::View, false));
        assert!(!is_allowed(Role::Guest, Action::View, true));
        assert!(!is_allowed(Role::Guest, Action::Edit, false));
        assert!(!is_allowed(Role::Guest, Action::Publish, false));
    }

    #[test]
    fn member_can_edit_and_publish_only_when_active() {
        assert!(is_allowed(Role::Member, Action::Edit, false));
        assert!(!is_allowed(Role::Member, Action::Edit, true));
        assert!(is_allowed(Role::Member, Action::Publish, false));
        assert!(!is_allowed(Role::Member, Action::Publish, true));
    }

    #[test]
    fn delete_is_admin_only_and_never_allowed_for_archived_items() {
        assert!(is_allowed(Role::Admin, Action::Delete, false));
        assert!(!is_allowed(Role::Admin, Action::Delete, true));
        assert!(!is_allowed(Role::Member, Action::Delete, false));
        assert!(!is_allowed(Role::Guest, Action::Delete, true));
    }

    #[test]
    fn admin_can_manage_active_items() {
        assert!(is_allowed(Role::Admin, Action::Edit, false));
        assert!(is_allowed(Role::Admin, Action::Publish, false));
        assert!(!is_allowed(Role::Admin, Action::Publish, true));
    }
}
