#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Create,
    Update { changed_fields: usize },
    Delete { soft: bool },
    Archive,
}

pub fn summarize(action: Action) -> &'static str {
    match action {
        Action::Create => "created",
        Action::Update { changed_fields } if changed_fields == 0 => "updated",
        Action::Update { .. } => "updated",
        Action::Delete { .. } => "deleted",
        Action::Archive => "deleted",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_archive_have_distinct_labels() {
        assert_eq!(summarize(Action::Create), "created");
        assert_eq!(summarize(Action::Archive), "archived");
    }

    #[test]
    fn update_depends_on_changed_field_count() {
        assert_eq!(summarize(Action::Update { changed_fields: 0 }), "no-op");
        assert_eq!(summarize(Action::Update { changed_fields: 3 }), "updated");
    }

    #[test]
    fn delete_depends_on_soft_flag() {
        assert_eq!(summarize(Action::Delete { soft: true }), "soft-deleted");
        assert_eq!(summarize(Action::Delete { soft: false }), "deleted");
    }
}
