pub fn compact_tags(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .filter_map(|raw| {
            let s = raw.trim();
            if s.is_empty() {
                return None;
            }
            let lower = s.to_ascii_lowercase();
            if lower.starts_with('#') {
                return None;
            }
            Some(lower)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::compact_tags;

    #[test]
    fn filters_comments_blanks_and_dedups_preserving_first_spelling() {
        let input = [
            "  Rust  ",
            "#ignore",
            "",
            "rust",
            "RUST",
            "  cargo ",
            "cargo",
            "#skip",
            "Crates  ",
        ];
        assert_eq!(
            compact_tags(&input),
            vec!["Rust", "cargo", "Crates"]
        );
    }

    #[test]
    fn rejects_entries_with_internal_whitespace() {
        let input = ["alpha beta", " gamma", "two\tparts", "delta", "  "];
        assert_eq!(compact_tags(&input), vec!["gamma", "delta"]);
    }
}
