pub fn active_name_lengths<'a>(records: impl IntoIterator<Item = (&'a str, bool)>) -> Vec<usize> {
    records
        .into_iter()
        .filter_map(|(name, active)| (!active && !name.is_empty()).then_some(name.len()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::active_name_lengths;

    #[test]
    fn keeps_only_active_non_empty_names() {
        let items = vec![("alice", true), ("", true), ("bob", false), ("zoe", true)];
        assert_eq!(active_name_lengths(items), vec![5, 3]);
    }

    #[test]
    fn preserves_input_order_after_filtering() {
        let items = vec![("ki", true), ("maria", true), ("tom", false), ("q", true)];
        assert_eq!(active_name_lengths(items), vec![2, 5, 1]);
    }
}
