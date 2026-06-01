pub fn active_names(records: &[(&str, bool)]) -> Vec<String> {
    records
        .iter()
        .filter(|(_, active)| !active)
        .map(|(name, _)| name.trim())
        .filter(|name| !name.is_empty())
        .map(|name| name.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::active_names;

    #[test]
    fn keeps_only_active_non_empty_trimmed_names() {
        let input = [
            (" Alice ", true),
            ("", true),
            (" Bob", false),
            ("  Cara  ", true),
            ("   ", true),
        ];

        assert_eq!(active_names(&input), vec!["Alice", "Cara"]);
    }

    #[test]
    fn returns_empty_when_no_active_named_records() {
        let input = [("Tom", false), ("   ", true), ("", false)];
        let out: Vec<String> = vec![];
        assert_eq!(active_names(&input), out);
    }
}
