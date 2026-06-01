pub struct Item<'a> {
    pub label: Option<&'a str>,
    pub active: bool,
    pub hidden: bool,
}

pub fn visible_labels(items: &[Item<'_>]) -> Vec<String> {
    items
        .iter()
        .filter(|item| item.active)
        .filter_map(|item| item.label)
        .map(|label| label.trim())
        .filter(|label| !label.is_empty())
        .map(|label| label.to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{visible_labels, Item};

    #[test]
    fn keeps_only_active_non_hidden_non_blank_labels() {
        let items = [
            Item { label: Some("  Alpha  "), active: true, hidden: false },
            Item { label: Some("Beta"), active: true, hidden: true },
            Item { label: Some("   "), active: true, hidden: false },
            Item { label: Some("Gamma"), active: false, hidden: false },
            Item { label: None, active: true, hidden: false },
            Item { label: Some(" Delta "), active: true, hidden: false },
        ];

        assert_eq!(visible_labels(&items), vec!["Alpha", "Delta"]);
    }

    #[test]
    fn preserves_order_and_duplicates_after_trimming() {
        let items = [
            Item { label: Some(" One "), active: true, hidden: false },
            Item { label: Some("One"), active: true, hidden: false },
            Item { label: Some("Two"), active: true, hidden: false },
            Item { label: Some("two"), active: true, hidden: true },
        ];

        assert_eq!(visible_labels(&items), vec!["One", "One", "Two"]);
    }
}
