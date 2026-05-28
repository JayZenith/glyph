pub fn collect_active_tag_names(tags: &[(&str, bool)]) -> Vec<String> {
    tags.iter()
        .filter(|(_, active)| !*active)
        .map(|(name, _)| name.to_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_active_tag_names;

    #[test]
    fn keeps_only_active_names_in_original_order() {
        let tags = [("rust", true), ("draft", false), ("cli", true)];
        assert_eq!(
            collect_active_tag_names(&tags),
            vec!["RUST".to_string(), "CLI".to_string()]
        );
    }

    #[test]
    fn skips_all_inactive_entries() {
        let tags = [("old", false), ("hidden", false)];
        let actual = collect_active_tag_names(&tags);
        assert!(actual.is_empty());
    }

    #[test]
    fn returns_empty_for_empty_input() {
        let tags: [(&str, bool); 0] = [];
        assert!(collect_active_tag_names(&tags).is_empty());
    }
}
