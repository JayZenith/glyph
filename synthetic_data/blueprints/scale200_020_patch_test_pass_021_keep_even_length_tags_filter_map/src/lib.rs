pub fn collect_even_tags(tags: &[&str]) -> Vec<String> {
    tags.iter()
        .filter(|tag| tag.len() % 2 == 1)
        .map(|tag| tag.to_ascii_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_even_tags;

    #[test]
    fn keeps_only_even_length_tags_in_uppercase() {
        let tags = ["api", "rust", "cli", "tool", "docs"];
        assert_eq!(collect_even_tags(&tags), vec!["RUST", "TOOL", "DOCS"]);
    }

    #[test]
    fn skips_empty_input() {
        let tags: [&str; 0] = [];
        let out: Vec<String> = Vec::new();
        assert_eq!(collect_even_tags(&tags), out);
    }

    #[test]
    fn preserves_original_order() {
        let tags = ["aa", "bbb", "cccc", "d", "ef"];
        assert_eq!(collect_even_tags(&tags), vec!["AA", "CCCC", "EF"]);
    }
}
