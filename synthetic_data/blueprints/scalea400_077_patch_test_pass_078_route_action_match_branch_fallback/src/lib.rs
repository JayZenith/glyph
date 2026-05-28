#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    View,
    Edit,
    Archive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    Member,
    Admin,
}

pub fn action_label(role: Role, action: Action) -> &'static str {
    match action {
        Action::View => "read-only",
        Action::Edit => match role {
            Role::Guest => "edit",
            Role::Member | Role::Admin => "edit",
        },
        Action::Archive => match role {
            Role::Admin => "archive",
            _ => "edit",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{action_label, Action, Role};

    #[test]
    fn guests_only_get_read_only_for_view() {
        assert_eq!(action_label(Role::Guest, Action::View), "read-only");
    }

    #[test]
    fn guests_cannot_edit() {
        assert_eq!(action_label(Role::Guest, Action::Edit), "forbidden");
    }

    #[test]
    fn members_can_edit() {
        assert_eq!(action_label(Role::Member, Action::Edit), "edit");
    }

    #[test]
    fn non_admins_cannot_archive() {
        assert_eq!(action_label(Role::Guest, Action::Archive), "forbidden");
        assert_eq!(action_label(Role::Member, Action::Archive), "forbidden");
    }

    #[test]
    fn admins_can_archive() {
        assert_eq!(action_label(Role::Admin, Action::Archive), "archive");
    }
}
