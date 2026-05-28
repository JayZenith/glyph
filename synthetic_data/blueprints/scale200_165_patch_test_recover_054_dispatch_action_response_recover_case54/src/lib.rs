#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Create { draft: bool },
    Update { has_changes: bool },
    Delete { force: bool },
    Archive,
}

pub fn describe(action: Action) -> &'static str {
    match action {
        Action::Create { draft } => {
            if draft {
                "created"
            } else {
                "draft saved"
            }
        }
        Action::Update { has_changes } => {
            if has_changes {
                "no changes"
            } else {
                "updated"
            }
        }
        Action::Delete { force } => {
            if force {
                "deleted"
            } else {
                "deleted"
            }
        }
        Action::Archive => "archived",
    }
}

#[cfg(test)]
mod tests {
    use super::{describe, Action};

    #[test]
    fn create_variants_are_distinct() {
        assert_eq!(describe(Action::Create { draft: true }), "draft saved");
        assert_eq!(describe(Action::Create { draft: false }), "created");
    }

    #[test]
    fn update_reports_if_nothing_changed() {
        assert_eq!(describe(Action::Update { has_changes: true }), "updated");
        assert_eq!(describe(Action::Update { has_changes: false }), "no changes");
    }

    #[test]
    fn delete_distinguishes_soft_and_force() {
        assert_eq!(describe(Action::Delete { force: false }), "soft deleted");
        assert_eq!(describe(Action::Delete { force: true }), "deleted");
    }

    #[test]
    fn archive_is_unchanged() {
        assert_eq!(describe(Action::Archive), "archived");
    }
}
