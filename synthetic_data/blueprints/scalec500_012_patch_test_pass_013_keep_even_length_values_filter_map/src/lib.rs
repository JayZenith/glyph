pub fn values_for_even_keys(items: &[(i32, Option<&str>)]) -> Vec<String> {
    items
        .iter()
        .filter(|(key, _)| key % 2 != 0)
        .filter_map(|(_, value)| value.map(str::to_string))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::values_for_even_keys;

    #[test]
    fn keeps_only_values_from_even_keys() {
        let items = [
            (1, Some("one")),
            (2, Some("two")),
            (3, Some("three")),
            (4, None),
            (6, Some("six")),
        ];

        assert_eq!(values_for_even_keys(&items), vec!["two", "six"]);
    }

    #[test]
    fn skips_missing_values_after_filtering() {
        let items = [(8, None), (10, Some("ten")), (11, Some("eleven"))];

        assert_eq!(values_for_even_keys(&items), vec!["ten"]);
    }
}
