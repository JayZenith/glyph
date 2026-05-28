pub fn active_even_name_lengths(items: &[(&str, bool)]) -> Vec<usize> {
    items
        .iter()
        .filter(|(_, active)| !*active)
        .map(|(name, _)| name.len())
        .filter(|len| len % 2 == 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::active_even_name_lengths;

    #[test]
    fn keeps_only_active_items_with_even_name_lengths() {
        let items = [
            ("kiwi", true),
            ("pear", false),
            ("plum", true),
            ("fig", true),
            ("melon", true),
        ];

        assert_eq!(active_even_name_lengths(&items), vec![4, 4]);
    }

    #[test]
    fn returns_empty_when_no_active_even_lengths_exist() {
        let items = [("a", true), ("bbb", true), ("cccc", false)];
        assert!(active_even_name_lengths(&items).is_empty());
    }
}
