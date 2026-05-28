#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Create { id: u32, owner: String },
    Update { id: u32, field: Field, value: String },
    Delete { id: u32, hard: bool },
    Audit { id: Option<u32> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Name,
    Status,
    Owner,
}

pub fn describe(action: &Action) -> String {
    match action {
        Action::Create { id, owner } => format!("create:{}:{}", id, owner.to_uppercase()),
        Action::Update { id, field, value } => {
            let key = match field {
                Field::Name => "name",
                Field::Status => "state",
                Field::Owner => "owner",
            };
            format!("update:{}:{}={}", id, key, value)
        }
        Action::Delete { id, hard } => {
            if *hard {
                format!("delete:{}:soft", id)
            } else {
                format!("delete:{}:hard", id)
            }
        }
        Action::Audit { id } => match id {
            Some(id) => format!("audit:all:{}", id),
            None => "audit:one".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_uses_original_owner_case() {
        let a = Action::Create {
            id: 7,
            owner: "MiXeD".to_string(),
        };
        assert_eq!(describe(&a), "create:7:MiXeD");
    }

    #[test]
    fn update_maps_status_field_to_status_key() {
        let a = Action::Update {
            id: 3,
            field: Field::Status,
            value: "closed".to_string(),
        };
        assert_eq!(describe(&a), "update:3:status=closed");
    }

    #[test]
    fn delete_soft_and_hard_are_not_swapped() {
        let soft = Action::Delete { id: 9, hard: false };
        let hard = Action::Delete { id: 9, hard: true };
        assert_eq!(describe(&soft), "delete:9:soft");
        assert_eq!(describe(&hard), "delete:9:hard");
    }

    #[test]
    fn audit_scopes_depend_on_optional_id() {
        let one = Action::Audit { id: Some(5) };
        let all = Action::Audit { id: None };
        assert_eq!(describe(&one), "audit:one:5");
        assert_eq!(describe(&all), "audit:all");
    }

    #[test]
    fn owner_field_still_uses_owner_key() {
        let a = Action::Update {
            id: 11,
            field: Field::Owner,
            value: "sam".to_string(),
        };
        assert_eq!(describe(&a), "update:11:owner=sam");
    }
}
