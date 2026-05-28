pub struct Item<'a> {
    pub name: &'a str,
    pub label: Option<&'a str>,
    pub archived: bool,
}

pub fn visible_labels(items: &[Item<'_>]) -> Vec<String> {
    items
        .iter()
        .filter_map(|item| {
            let text = item.label.unwrap_or(item.name).trim();
            if text.is_empty() {
                None
            } else {
                Some(text.to_string())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{visible_labels, Item};

    #[test]
    fn skips_archived_items() {
        let items = [
            Item {
                name: "alpha",
                label: Some("Featured"),
                archived: false,
            },
            Item {
                name: "beta",
                label: Some("Legacy"),
                archived: true,
            },
            Item {
                name: "gamma",
                label: None,
                archived: false,
            },
        ];

        assert_eq!(visible_labels(&items), vec!["Featured", "gamma"]);
    }

    #[test]
    fn falls_back_to_name_when_label_is_blank() {
        let items = [
            Item {
                name: "primary",
                label: Some("   "),
                archived: false,
            },
            Item {
                name: "secondary",
                label: Some(" Ready "),
                archived: false,
            },
        ];

        assert_eq!(visible_labels(&items), vec!["primary", "Ready"]);
    }

    #[test]
    fn drops_items_when_both_label_and_name_are_blank() {
        let items = [
            Item {
                name: "   ",
                label: None,
                archived: false,
            },
            Item {
                name: "kept",
                label: None,
                archived: false,
            },
        ];

        assert_eq!(visible_labels(&items), vec!["kept"]);
    }
}
