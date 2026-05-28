pub struct Item {
    pub code: Option<String>,
    pub active: bool,
}

pub fn collect_short_codes(items: &[Item]) -> Vec<String> {
    items
        .iter()
        .filter_map(|item| item.code.as_deref())
        .filter(|code| !code.is_empty() && code.len() <= 3)
        .map(str::to_uppercase)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(code: Option<&str>, active: bool) -> Item {
        Item {
            code: code.map(str::to_string),
            active,
        }
    }

    #[test]
    fn keeps_only_active_short_nonempty_codes() {
        let items = vec![
            item(Some("ab"), true),
            item(Some("xyz"), true),
            item(Some("toolong"), true),
            item(Some("cd"), false),
            item(Some(""), true),
            item(None, true),
        ];

        assert_eq!(collect_short_codes(&items), vec!["AB", "XYZ"]);
    }

    #[test]
    fn preserves_input_order_after_filtering() {
        let items = vec![
            item(Some("b"), true),
            item(Some("a"), false),
            item(Some("cc"), true),
        ];

        assert_eq!(collect_short_codes(&items), vec!["B", "CC"]);
    }
}
