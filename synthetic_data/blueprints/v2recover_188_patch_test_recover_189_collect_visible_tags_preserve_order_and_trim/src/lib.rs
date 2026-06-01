pub fn collect_visible_tags(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .filter_map(|raw| {
            let s = raw.trim();
            if s.is_empty() || s.starts_with('#') {
                return None;
            }
            let name = s.strip_prefix("tag:")?;
            let value = name.trim().to_lowercase();
            if value.is_empty() {
                None
            } else {
                Some(value)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_visible_tags;

    #[test]
    fn skips_comments_and_blank_lines() {
        let input = ["", "   ", "# hidden", "tag: Rust", " note", "tag: Tools "];
        assert_eq!(collect_visible_tags(&input), vec!["rust", "tools"]);
    }

    #[test]
    fn drops_private_tags_and_dedups_preserving_first_visible_order() {
        let input = [
            "tag: Rust",
            "tag: private-internal",
            "tag: tools",
            "tag: rust",
            "tag: PRIVATE-team",
            "tag: tools  ",
            "tag: data",
        ];
        assert_eq!(collect_visible_tags(&input), vec!["rust", "tools", "data"]);
    }

    #[test]
    fn accepts_space_after_prefix_and_ignores_empty_names() {
        let input = ["tag:   Alpha", "tag:", "tag:   ", "tag: beta "];
        assert_eq!(collect_visible_tags(&input), vec!["alpha", "beta"]);
    }
}
