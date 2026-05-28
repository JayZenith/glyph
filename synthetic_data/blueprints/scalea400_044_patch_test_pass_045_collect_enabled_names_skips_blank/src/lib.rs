pub struct User<'a> {
    pub name: &'a str,
    pub enabled: bool,
}

pub fn enabled_names(users: &[User<'_>]) -> Vec<String> {
    users
        .iter()
        .filter(|u| u.enabled)
        .map(|u| u.name.trim())
        .filter(|name| !name.is_empty())
        .map(|name| name.to_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{enabled_names, User};

    #[test]
    fn keeps_enabled_non_blank_names_in_original_form() {
        let users = [
            User { name: "Alice", enabled: true },
            User { name: "", enabled: true },
            User { name: "Bob", enabled: false },
            User { name: "  Cara  ", enabled: true },
        ];

        assert_eq!(enabled_names(&users), vec!["Alice", "Cara"]);
    }

    #[test]
    fn preserves_order_after_filtering() {
        let users = [
            User { name: "zed", enabled: true },
            User { name: "amy", enabled: true },
            User { name: "", enabled: true },
        ];

        assert_eq!(enabled_names(&users), vec!["zed", "amy"]);
    }
}
