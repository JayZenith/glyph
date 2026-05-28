#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Draft,
    Review,
    Published,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Guest,
    Author,
    Reviewer,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    View,
    Edit,
    Approve,
    Restore,
}

pub fn route_action(stage: Stage, role: Role, action: Action) -> &'static str {
    match action {
        Action::View => match stage {
            Stage::Archived => match role {
                Role::Admin => "archive_view",
                _ => "forbidden",
            },
            _ => "view",
        },
        Action::Edit => match stage {
            Stage::Draft => match role {
                Role::Author | Role::Admin => "edit_draft",
                _ => "forbidden",
            },
            Stage::Review => match role {
                Role::Reviewer => "review_note",
                Role::Admin => "edit_review",
                _ => "forbidden",
            },
            Stage::Published => "forbidden",
            Stage::Archived => "forbidden",
        },
        Action::Approve => match stage {
            Stage::Review => match role {
                Role::Admin => "approve",
                _ => "forbidden",
            },
            _ => "forbidden",
        },
        Action::Restore => match stage {
            Stage::Archived => match role {
                Role::Admin => "restore",
                _ => "forbidden",
            },
            _ => "forbidden",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guest_cannot_view_archived_items() {
        assert_eq!(route_action(Stage::Archived, Role::Guest, Action::View), "forbidden");
    }

    #[test]
    fn admin_has_special_archived_view() {
        assert_eq!(route_action(Stage::Archived, Role::Admin, Action::View), "archive_view");
    }

    #[test]
    fn reviewer_can_edit_review_stage() {
        assert_eq!(route_action(Stage::Review, Role::Reviewer, Action::Edit), "edit_review");
    }

    #[test]
    fn reviewer_can_approve_review_stage() {
        assert_eq!(route_action(Stage::Review, Role::Reviewer, Action::Approve), "approve");
    }

    #[test]
    fn author_cannot_approve_review_stage() {
        assert_eq!(route_action(Stage::Review, Role::Author, Action::Approve), "forbidden");
    }

    #[test]
    fn published_items_are_viewable() {
        assert_eq!(route_action(Stage::Published, Role::Guest, Action::View), "view");
    }

    #[test]
    fn admin_can_restore_archived_items() {
        assert_eq!(route_action(Stage::Archived, Role::Admin, Action::Restore), "restore");
    }
}
