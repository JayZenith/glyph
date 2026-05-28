pub fn collect_tags(input: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = input
        .iter()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::collect_tags;

    #[test]
    fn drops_blank_and_lowercases() {
        let got = collect_tags(&["  Rust ", "", "CLI", "  "]);
        assert_eq!(got, vec!["cli", "rust"]);
    }

    #[test]
    fn skips_tags_starting_with_hash() {
        let got = collect_tags(&["Rust", "#draft", "cli", "#internal"]);
        assert_eq!(got, vec!["cli", "rust"]);
    }

    #[test]
    fn dedups_after_normalization_and_sorts() {
        let got = collect_tags(&[" Beta", "alpha", "ALPHA", "beta ", "gamma"]);
        assert_eq!(got, vec!["alpha", "beta", "gamma"]);
    }
}
