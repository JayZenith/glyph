pub fn collect_tags(input: &[&str]) -> Vec<String> {
    input
        .iter()
        .filter_map(|raw| {
            let part = raw.strip_prefix("tag:")?;
            let trimmed = part.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_lowercase())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_tags;

    #[test]
    fn keeps_only_valid_tags_and_normalizes_case() {
        let input = [
            "note:skip",
            "tag: Rust ",
            "tag:",
            "tag:API",
            "misc",
            "tag:  cli-tools  ",
        ];
        assert_eq!(
            collect_tags(&input),
            vec!["rust", "api", "cli-tools"]
        );
    }

    #[test]
    fn drops_tags_with_internal_whitespace() {
        let input = ["tag:two words", "tag:ok", "tag: spaced out "];
        assert_eq!(collect_tags(&input), vec!["ok"]);
    }

    #[test]
    fn deduplicates_case_insensitively_after_normalization() {
        let input = ["tag:Rust", "tag: rust ", "tag:RUST", "tag:web"];
        assert_eq!(collect_tags(&input), vec!["rust", "web"]);
    }
}
