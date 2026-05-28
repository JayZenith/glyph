pub fn active_tag_lengths(tags: &[(&str, bool)]) -> Vec<usize> {
    tags.iter()
        .filter(|(_, active)| !*active)
        .map(|(name, _)| name.len())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::active_tag_lengths;

    #[test]
    fn keeps_only_active_tags_in_order() {
        let tags = [
            ("red", true),
            ("blue", false),
            ("green", true),
            ("id", true),
        ];

        assert_eq!(active_tag_lengths(&tags), vec![3, 5, 2]);
    }

    #[test]
    fn returns_empty_when_nothing_is_active() {
        let tags = [("one", false), ("two", false)];
        assert!(active_tag_lengths(&tags).is_empty());
    }
}
