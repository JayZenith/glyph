pub struct User<'a> {
    pub name: &'a str,
    pub active: bool,
}

pub fn active_names(users: &[User<'_>]) -> Vec<String> {
    users
        .iter()
        .filter(|u| !u.active)
        .map(|u| u.name.to_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_only_active_users_in_original_order() {
        let users = [
            User {
                name: "alice",
                active: true,
            },
            User {
                name: "bob",
                active: false,
            },
            User {
                name: "carol",
                active: true,
            },
        ];

        assert_eq!(active_names(&users), vec!["ALICE", "CAROL"]);
    }

    #[test]
    fn returns_empty_when_no_active_users() {
        let users = [
            User {
                name: "bob",
                active: false,
            },
            User {
                name: "dave",
                active: false,
            },
        ];

        let names: Vec<String> = Vec::new();
        assert_eq!(active_names(&users), names);
    }
}
