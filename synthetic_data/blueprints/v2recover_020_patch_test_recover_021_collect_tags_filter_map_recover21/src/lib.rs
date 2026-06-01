pub fn collect_tags(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .map(|part| part.to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_tags;

    #[test]
    fn skips_comments_and_dedups_case_insensitively() {
        let got = collect_tags("Rust, #ignore, rust, TOKIO, tokio, serde");
        assert_eq!(got, vec!["rust", "tokio", "serde"]);
    }

    #[test]
    fn ignores_blank_and_invalid_entries_but_keeps_hyphenated() {
        let got = collect_tags(" alpha , , beta-2, bad tag, _hidden, gamma!, delta ");
        assert_eq!(got, vec!["alpha", "beta-2", "delta"]);
    }
}
