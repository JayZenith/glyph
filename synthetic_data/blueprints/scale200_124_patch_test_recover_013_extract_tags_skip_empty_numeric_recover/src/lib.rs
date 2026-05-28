pub fn extract_tags(input: &[&str]) -> Vec<String> {
    input
        .iter()
        .filter_map(|raw| raw.strip_prefix("tag:"))
        .map(|tag| tag.trim().to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::extract_tags;

    #[test]
    fn keeps_only_tag_entries_and_normalizes_case() {
        let items = ["note:skip", "tag:Rust", "tag: WEB ", "misc", "tag:db"];
        assert_eq!(extract_tags(&items), vec!["rust", "web", "db"]);
    }

    #[test]
    fn skips_empty_and_numeric_tags_after_trim() {
        let items = ["tag:   ", "tag:42", "tag:r2d2", "tag: api ", "tag:007"];
        assert_eq!(extract_tags(&items), vec!["r2d2", "api"]);
    }
}
