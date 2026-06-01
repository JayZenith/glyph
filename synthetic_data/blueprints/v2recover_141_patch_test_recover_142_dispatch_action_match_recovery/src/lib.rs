#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Create { name: String, urgent: bool },
    Delete { id: u32, hard: bool },
    Pause,
    Resume,
}

pub fn describe(action: Action) -> String {
    match action {
        Action::Create { name, urgent } => {
            if urgent {
                format!("create:{}", name)
            } else {
                format!("create:{}:urgent", name)
            }
        }
        Action::Delete { id, hard } => {
            if hard {
                format!("delete:{}", id)
            } else {
                format!("delete:{}:hard", id)
            }
        }
        Action::Pause => "resume".to_string(),
        Action::Resume => "pause".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_variants_are_labeled_correctly() {
        assert_eq!(
            describe(Action::Create {
                name: "alpha".into(),
                urgent: false,
            }),
            "create:alpha"
        );
        assert_eq!(
            describe(Action::Create {
                name: "beta".into(),
                urgent: true,
            }),
            "create:beta:urgent"
        );
    }

    #[test]
    fn delete_variants_include_hard_suffix_only_when_needed() {
        assert_eq!(describe(Action::Delete { id: 7, hard: false }), "delete:7");
        assert_eq!(describe(Action::Delete { id: 9, hard: true }), "delete:9:hard");
    }

    #[test]
    fn pause_and_resume_are_distinct() {
        assert_eq!(describe(Action::Pause), "pause");
        assert_eq!(describe(Action::Resume), "resume");
    }
}
