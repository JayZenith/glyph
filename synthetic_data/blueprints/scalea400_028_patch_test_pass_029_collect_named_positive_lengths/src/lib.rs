pub fn named_positive_lengths(items: &[(&str, i32)]) -> Vec<usize> {
    items
        .iter()
        .filter(|(_, qty)| *qty >= 0)
        .map(|(name, _)| name.len())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::named_positive_lengths;

    #[test]
    fn keeps_only_positive_quantities() {
        let items = [("apple", 3), ("pear", 0), ("fig", -2), ("melon", 5)];
        assert_eq!(named_positive_lengths(&items), vec![5, 5]);
    }

    #[test]
    fn preserves_order_after_filtering() {
        let items = [("kiwi", 1), ("plum", -1), ("banana", 2)];
        assert_eq!(named_positive_lengths(&items), vec![4, 6]);
    }

    #[test]
    fn empty_when_nothing_matches() {
        let items = [("a", 0), ("bb", -3)];
        assert!(named_positive_lengths(&items).is_empty());
    }
}
