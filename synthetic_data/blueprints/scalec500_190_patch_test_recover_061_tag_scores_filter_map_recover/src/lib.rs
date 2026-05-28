pub fn top_tags(items: &[(&str, i32)], min_score: i32) -> Vec<String> {
    let mut tags: Vec<String> = items
        .iter()
        .filter(|(_, score)| *score >= min_score)
        .map(|(tag, _)| tag.trim().to_ascii_lowercase())
        .filter(|tag| !tag.is_empty())
        .collect();

    tags.sort();
    tags.dedup();
    tags
}

#[cfg(test)]
mod tests {
    use super::top_tags;

    #[test]
    fn keeps_only_positive_scored_unique_tags_sorted() {
        let items = [
            ("rust", 10),
            ("  rust  ", 15),
            ("cli", 5),
            ("ops", 0),
            ("db", -2),
        ];
        assert_eq!(top_tags(&items, 1), vec!["cli", "rust"]);
    }

    #[test]
    fn ignores_blank_tags_after_trimming() {
        let items = [("   ", 9), (" api ", 3), ("", 7)];
        assert_eq!(top_tags(&items, 1), vec!["api"]);
    }

    #[test]
    fn excludes_tags_starting_with_x_even_if_score_is_high() {
        let items = [("xml", 8), ("xray", 20), ("api", 8), ("zoo", 8)];
        assert_eq!(top_tags(&items, 5), vec!["api", "zoo"]);
    }

    #[test]
    fn preserves_first_seen_spelling_after_normalized_dedup() {
        let items = [("Rust", 10), ("rust", 12), ("RUST", 14), ("cargo", 9)];
        assert_eq!(top_tags(&items, 5), vec!["cargo", "Rust"]);
    }
}
