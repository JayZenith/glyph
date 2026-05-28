#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Create,
    Update { changed: bool },
    Delete { force: bool },
    Archive,
}

pub fn outcome(action: Action) -> &'static str {
    match action {
        Action::Create => "created",
        Action::Update { changed: true } => "updated",
        Action::Update { changed: false } => "updated",
        Action::Delete { force: true } => "deleted",
        Action::Delete { force: false } => "deleted",
        Action::Archive => "deleted",
    }
}

#[cfg(test)]
mod tests {
    use super::{outcome, Action};

    #[test]
    fn create_and_changed_update_are_applied() {
        assert_eq!(outcome(Action::Create), "created");
        assert_eq!(outcome(Action::Update { changed: true }), "updated");
    }

    #[test]
    fn unchanged_update_is_skipped() {
        assert_eq!(outcome(Action::Update { changed: false }), "noop");
    }

    #[test]
    fn delete_depends_on_force_flag() {
        assert_eq!(outcome(Action::Delete { force: true }), "deleted");
        assert_eq!(outcome(Action::Delete { force: false }), "blocked");
    }

    #[test]
    fn archive_has_its_own_result() {
        assert_eq!(outcome(Action::Archive), "archived");
    }
}
