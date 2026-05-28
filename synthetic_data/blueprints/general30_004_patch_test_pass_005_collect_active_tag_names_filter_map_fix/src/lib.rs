pub fn collect_active_tag_names(items: &[(&str, bool)]) -> Vec<String> {
    items
        .iter()
        .filter(|(_, active)| !*active)
        .map(|(name, _)| name.to_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_active_tag_names;

    #[test]
    fn keeps_only_active_items_in_original_order() {
        let items = [("red", true), ("blue", false), ("green", true)];
        assert_eq!(collect_active_tag_names(&items), vec!["RED", "GREEN"]);
    }

    #[test]
    fn returns_empty_when_nothing_is_active() {
        let items = [("red", false), ("blue", false)];
        let result: Vec<String> = Vec::new();
        assert_eq!(collect_active_tag_names(&items), result);
    }
}
