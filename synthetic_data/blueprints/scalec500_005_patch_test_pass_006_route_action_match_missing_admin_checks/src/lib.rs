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

pub fn can_perform(role: Role, action: Action, owns_post: bool) -> bool {
    match action {
        Action::View => true,
        Action::Edit => role != Role::Guest,
        Action::Delete => owns_post,
        Action::Publish => role == Role::Member,
    }
}

#[cfg(test)]
mod tests {
    use super::{can_perform, Action, Role};

    #[test]
    fn guests_can_only_view() {
        assert!(can_perform(Role::Guest, Action::View, false));
        assert!(!can_perform(Role::Guest, Action::Edit, true));
        assert!(!can_perform(Role::Guest, Action::Delete, true));
        assert!(!can_perform(Role::Guest, Action::Publish, false));
    }

    #[test]
    fn members_can_edit_and_delete_only_their_own_posts() {
        assert!(can_perform(Role::Member, Action::Edit, false));
        assert!(can_perform(Role::Member, Action::Delete, true));
        assert!(!can_perform(Role::Member, Action::Delete, false));
        assert!(can_perform(Role::Member, Action::Publish, false));
    }

    #[test]
    fn admins_can_edit_delete_any_post_and_publish() {
        assert!(can_perform(Role::Admin, Action::Edit, false));
        assert!(can_perform(Role::Admin, Action::Delete, true));
        assert!(can_perform(Role::Admin, Action::Delete, false));
        assert!(can_perform(Role::Admin, Action::Publish, false));
    }
}
