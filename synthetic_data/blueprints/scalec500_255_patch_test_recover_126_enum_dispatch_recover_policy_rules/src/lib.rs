#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    View,
    Edit,
    Delete,
    Share,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    Member,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Document { archived: bool, owner: bool },
    Folder { archived: bool },
    AdminPanel,
}

pub fn is_allowed(role: Role, action: Action, resource: Resource) -> bool {
    match resource {
        Resource::AdminPanel => match role {
            Role::Admin => matches!(action, Action::View | Action::Edit),
            _ => false,
        },
        Resource::Folder { archived } => match role {
            Role::Guest => matches!(action, Action::View),
            Role::Member => {
                if archived {
                    matches!(action, Action::View | Action::Edit)
                } else {
                    matches!(action, Action::View | Action::Edit | Action::Share)
                }
            }
            Role::Admin => !matches!(action, Action::Delete),
        },
        Resource::Document { archived, owner } => match role {
            Role::Guest => matches!(action, Action::View | Action::Share),
            Role::Member => {
                if owner {
                    if archived {
                        matches!(action, Action::View | Action::Edit | Action::Delete)
                    } else {
                        true
                    }
                } else if archived {
                    matches!(action, Action::View)
                } else {
                    matches!(action, Action::View | Action::Edit)
                }
            }
            Role::Admin => {
                if archived {
                    matches!(action, Action::View | Action::Edit)
                } else {
                    true
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guest_cannot_share_documents() {
        assert!(!is_allowed(
            Role::Guest,
            Action::Share,
            Resource::Document {
                archived: false,
                owner: false,
            }
        ));
    }

    #[test]
    fn member_owner_cannot_delete_archived_document() {
        assert!(!is_allowed(
            Role::Member,
            Action::Delete,
            Resource::Document {
                archived: true,
                owner: true,
            }
        ));
    }

    #[test]
    fn admin_can_delete_folder() {
        assert!(is_allowed(
            Role::Admin,
            Action::Delete,
            Resource::Folder { archived: false }
        ));
    }

    #[test]
    fn admin_can_view_admin_panel() {
        assert!(is_allowed(Role::Admin, Action::View, Resource::AdminPanel));
    }

    #[test]
    fn admin_cannot_delete_admin_panel() {
        assert!(!is_allowed(
            Role::Admin,
            Action::Delete,
            Resource::AdminPanel
        ));
    }

    #[test]
    fn member_non_owner_cannot_edit_archived_document() {
        assert!(!is_allowed(
            Role::Member,
            Action::Edit,
            Resource::Document {
                archived: true,
                owner: false,
            }
        ));
    }

    #[test]
    fn member_can_share_active_folder() {
        assert!(is_allowed(
            Role::Member,
            Action::Share,
            Resource::Folder { archived: false }
        ));
    }
}
