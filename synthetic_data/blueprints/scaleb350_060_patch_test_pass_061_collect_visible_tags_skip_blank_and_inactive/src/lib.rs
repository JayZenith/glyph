pub struct Item<'a> {
    pub enabled: bool,
    pub hidden: bool,
    pub tag: Option<&'a str>,
}

pub fn collect_visible_tags(items: &[Item<'_>]) -> Vec<String> {
    items
        .iter()
        .filter(|item| item.enabled || !item.hidden)
        .filter_map(|item| item.tag)
        .map(|tag| tag.trim().to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{collect_visible_tags, Item};

    #[test]
    fn keeps_only_enabled_and_visible_non_blank_tags() {
        let items = [
            Item {
                enabled: true,
                hidden: false,
                tag: Some("  Alpha  "),
            },
            Item {
                enabled: false,
                hidden: false,
                tag: Some("Beta"),
            },
            Item {
                enabled: true,
                hidden: true,
                tag: Some("Gamma"),
            },
            Item {
                enabled: true,
                hidden: false,
                tag: Some("   "),
            },
            Item {
                enabled: true,
                hidden: false,
                tag: None,
            },
        ];

        let tags = collect_visible_tags(&items);
        assert_eq!(tags, vec!["alpha".to_string()]);
    }

    #[test]
    fn preserves_order_after_filtering() {
        let items = [
            Item {
                enabled: true,
                hidden: false,
                tag: Some(" One"),
            },
            Item {
                enabled: true,
                hidden: false,
                tag: Some("Two "),
            },
            Item {
                enabled: true,
                hidden: true,
                tag: Some("Three"),
            },
            Item {
                enabled: true,
                hidden: false,
                tag: Some("FOUR"),
            },
        ];

        let tags = collect_visible_tags(&items);
        assert_eq!(tags, vec!["one", "two", "four"]);
    }
}
