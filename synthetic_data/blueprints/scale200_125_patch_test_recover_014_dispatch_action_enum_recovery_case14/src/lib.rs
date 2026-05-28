#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    View { archived: bool },
    Edit { user_can_edit: bool, locked: bool },
    Delete { is_admin: bool, soft: bool },
    Share { external: bool, confirmed: bool },
}

pub fn decision(action: Action) -> &'static str {
    match action {
        Action::View { archived } => {
            if archived {
                "view"
            } else {
                "open"
            }
        }
        Action::Edit {
            user_can_edit,
            locked,
        } => {
            if user_can_edit {
                "edit"
            } else if locked {
                "request_unlock"
            } else {
                "readonly"
            }
        }
        Action::Delete { is_admin, soft } => {
            if is_admin || soft {
                "delete"
            } else {
                "forbidden"
            }
        }
        Action::Share {
            external,
            confirmed,
        } => {
            if external {
                "share_link"
            } else if confirmed {
                "share_internal"
            } else {
                "confirm_share"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view_branch_distinguishes_archived_items() {
        assert_eq!(decision(Action::View { archived: false }), "view");
        assert_eq!(decision(Action::View { archived: true }), "open_archived");
    }

    #[test]
    fn edit_branch_prioritizes_lock_before_permissions() {
        assert_eq!(
            decision(Action::Edit {
                user_can_edit: true,
                locked: true,
            }),
            "request_unlock"
        );
        assert_eq!(
            decision(Action::Edit {
                user_can_edit: true,
                locked: false,
            }),
            "edit"
        );
        assert_eq!(
            decision(Action::Edit {
                user_can_edit: false,
                locked: false,
            }),
            "readonly"
        );
    }

    #[test]
    fn delete_branch_only_allows_admin_hard_delete_or_any_soft_delete() {
        assert_eq!(
            decision(Action::Delete {
                is_admin: true,
                soft: false,
            }),
            "delete"
        );
        assert_eq!(
            decision(Action::Delete {
                is_admin: false,
                soft: true,
            }),
            "trash"
        );
        assert_eq!(
            decision(Action::Delete {
                is_admin: false,
                soft: false,
            }),
            "forbidden"
        );
    }

    #[test]
    fn share_branch_requires_confirmation_for_external_and_internal() {
        assert_eq!(
            decision(Action::Share {
                external: true,
                confirmed: false,
            }),
            "confirm_share"
        );
        assert_eq!(
            decision(Action::Share {
                external: true,
                confirmed: true,
            }),
            "share_link"
        );
        assert_eq!(
            decision(Action::Share {
                external: false,
                confirmed: true,
            }),
            "share_internal"
        );
    }
}
