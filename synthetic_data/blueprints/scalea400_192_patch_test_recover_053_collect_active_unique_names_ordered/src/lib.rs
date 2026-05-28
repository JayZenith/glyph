use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub name: String,
    pub active: bool,
    pub score: i32,
}

pub fn selected_names(users: &[User], min_score: i32) -> Vec<String> {
    let mut seen = HashSet::new();
    users
        .iter()
        .filter(|u| u.active || u.score >= min_score)
        .map(|u| u.name.trim().to_string())
        .filter(|name| !name.is_empty())
        .filter(|name| seen.insert(name.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(name: &str, active: bool, score: i32) -> User {
        User {
            name: name.to_string(),
            active,
            score,
        }
    }

    #[test]
    fn keeps_only_active_users_meeting_min_score_and_dedups_after_trim() {
        let users = vec![
            user("  Ada  ", true, 12),
            user("Ada", true, 50),
            user("Bob", true, 9),
            user("Cara", false, 20),
            user("", true, 30),
        ];

        assert_eq!(selected_names(&users, 10), vec!["Ada"]);
    }

    #[test]
    fn preserves_first_seen_order_for_unique_names() {
        let users = vec![
            user("Zoe", true, 15),
            user("Mia", true, 10),
            user("Zoe", true, 18),
            user("Ian", true, 14),
        ];

        assert_eq!(selected_names(&users, 10), vec!["Zoe", "Mia", "Ian"]);
    }
}
