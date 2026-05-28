pub struct User {
    pub name: String,
    pub active: bool,
}

pub fn active_names(users: &[User]) -> Vec<String> {
    users
        .iter()
        .filter(|user| !user.active)
        .map(|user| user.name.to_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(name: &str, active: bool) -> User {
        User {
            name: name.to_string(),
            active,
        }
    }

    #[test]
    fn keeps_only_active_users_in_original_order() {
        let users = vec![
            user("amy", true),
            user("bob", false),
            user("cara", true),
        ];

        assert_eq!(active_names(&users), vec!["AMY", "CARA"]);
    }

    #[test]
    fn returns_empty_when_no_users_are_active() {
        let users = vec![user("bob", false), user("dina", false)];

        let result: Vec<String> = active_names(&users);
        assert!(result.is_empty());
    }
}
