#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub name: Option<String>,
    pub active: bool,
}

pub fn active_names(users: &[User]) -> Vec<String> {
    users
        .iter()
        .filter(|u| u.name.is_some())
        .filter_map(|u| if u.active { None } else { u.name.clone() })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(name: Option<&str>, active: bool) -> User {
        User {
            name: name.map(str::to_string),
            active,
        }
    }

    #[test]
    fn keeps_only_active_users_with_names() {
        let users = vec![
            user(Some("Ana"), true),
            user(Some("Ben"), false),
            user(None, true),
            user(Some("Cara"), true),
        ];

        assert_eq!(active_names(&users), vec!["Ana", "Cara"]);
    }

    #[test]
    fn preserves_input_order() {
        let users = vec![
            user(Some("Zoe"), true),
            user(Some("Max"), true),
            user(Some("Ian"), false),
        ];

        assert_eq!(active_names(&users), vec!["Zoe", "Max"]);
    }
}
