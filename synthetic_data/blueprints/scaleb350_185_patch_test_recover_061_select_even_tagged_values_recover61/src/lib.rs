pub fn select_values(items: &[(&str, i32)]) -> Vec<String> {
    items
        .iter()
        .filter(|(_, value)| *value > 0)
        .map(|(name, value)| (name.trim(), *value))
        .filter(|(name, _)| !name.is_empty())
        .map(|(name, value)| format!("{}={}", name, value))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::select_values;

    #[test]
    fn keeps_only_positive_even_values_and_trims_names() {
        let items = [
            ("  alpha ", 2),
            ("beta", 3),
            ("", 8),
            (" gamma", -4),
            ("delta ", 6),
        ];

        assert_eq!(
            select_values(&items),
            vec!["alpha=2".to_string(), "delta=6".to_string()]
        );
    }

    #[test]
    fn preserves_input_order_and_drops_zero_or_odd_values() {
        let items = [
            ("one", 0),
            (" two ", 10),
            ("three", 7),
            (" four", 12),
            ("five ", 9),
        ];

        assert_eq!(
            select_values(&items),
            vec!["two=10".to_string(), "four=12".to_string()]
        );
    }
}
