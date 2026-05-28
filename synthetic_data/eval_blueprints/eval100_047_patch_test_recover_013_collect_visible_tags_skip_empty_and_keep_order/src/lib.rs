pub struct Item<'a> {
    pub active: bool,
    pub tag: Option<&'a str>,
}

pub fn collect_visible_tags(items: &[Item<'_>]) -> Vec<String> {
    items
        .iter()
        .filter_map(|item| item.tag.map(str::to_string))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_inactive_and_missing_tags() {
        let items = [
            Item { active: true, tag: Some("alpha") },
            Item { active: false, tag: Some("hidden") },
            Item { active: true, tag: None },
            Item { active: true, tag: Some("beta") },
        ];

        assert_eq!(collect_visible_tags(&items), vec!["alpha", "beta"]);
    }

    #[test]
    fn trims_and_ignores_empty_results() {
        let items = [
            Item { active: true, tag: Some("  red ") },
            Item { active: true, tag: Some("   ") },
            Item { active: true, tag: Some("") },
            Item { active: true, tag: Some("blue") },
        ];

        assert_eq!(collect_visible_tags(&items), vec!["red", "blue"]);
    }

    #[test]
    fn preserves_input_order() {
        let items = [
            Item { active: true, tag: Some("zeta") },
            Item { active: true, tag: Some("alpha") },
            Item { active: false, tag: Some("beta") },
            Item { active: true, tag: Some("gamma") },
        ];

        assert_eq!(collect_visible_tags(&items), vec!["zeta", "alpha", "gamma"]);
    }
}
