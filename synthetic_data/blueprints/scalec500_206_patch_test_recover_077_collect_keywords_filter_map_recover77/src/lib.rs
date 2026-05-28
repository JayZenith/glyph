pub fn collect_keywords(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }

            let (key, value) = line.split_once('=')?;
            let key = key.trim();
            let value = value.trim();

            if key != "tag" {
                return None;
            }

            if value.is_empty() {
                return None;
            }

            Some(value.to_ascii_lowercase())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_keywords;

    #[test]
    fn ignores_comments_blanks_and_other_keys() {
        let lines = [
            "",
            "  # skip this",
            "tag = Rust",
            "name = tool",
            "tag=CLI",
            " tag = ",
        ];
        assert_eq!(collect_keywords(&lines), vec!["rust", "cli"]);
    }

    #[test]
    fn drops_disabled_and_dedups_case_insensitively_in_order() {
        let lines = [
            "tag = Rust",
            "tag = rust",
            "tag = disabled",
            "tag = CLI",
            "tag = Disabled",
            "tag = cli",
            "tag = api",
        ];
        assert_eq!(collect_keywords(&lines), vec!["rust", "cli", "api"]);
    }
}
