pub struct User<'a> {
    pub name: &'a str,
    pub active: bool,
}

pub fn collect_active_names(users: &[User<'_>]) -> Vec<String> {
    users
        .iter()
        .filter(|user| user.active)
        .map(|user| user.name.trim())
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{collect_active_names, User};

    #[test]
    fn keeps_active_names_in_order() {
        let users = [
            User {
                name: " Ada ",
                active: true,
            },
            User {
                name: "Grace",
                active: false,
            },
            User {
                name: " Linus ",
                active: true,
            },
        ];

        assert_eq!(collect_active_names(&users), vec!["Ada", "Linus"]);
    }

    #[test]
    fn skips_empty_names_after_trimming() {
        let users = [
            User {
                name: "   ",
                active: true,
            },
            User {
                name: " Bob ",
                active: true,
            },
            User {
                name: "",
                active: true,
            },
        ];

        assert_eq!(collect_active_names(&users), vec!["Bob"]);
    }
}
