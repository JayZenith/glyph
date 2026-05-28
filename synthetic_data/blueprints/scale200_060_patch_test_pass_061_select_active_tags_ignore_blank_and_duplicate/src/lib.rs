pub fn selected_tags(rows: &[(&str, bool)]) -> Vec<String> {
    rows.iter()
        .filter(|(_, active)| *active)
        .map(|(tag, _)| tag.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::selected_tags;

    #[test]
    fn keeps_only_active_tags_in_order() {
        let rows = [
            ("ops", true),
            ("dev", false),
            ("qa", true),
        ];
        assert_eq!(selected_tags(&rows), vec!["ops", "qa"]);
    }

    #[test]
    fn ignores_blank_tags_after_trimming() {
        let rows = [
            ("  ", true),
            (" alpha ", true),
            ("", true),
            ("beta", true),
        ];
        assert_eq!(selected_tags(&rows), vec!["alpha", "beta"]);
    }

    #[test]
    fn removes_later_duplicates_after_trim() {
        let rows = [
            (" alpha ", true),
            ("beta", true),
            ("alpha", true),
            ("beta ", true),
            ("gamma", true),
        ];
        assert_eq!(selected_tags(&rows), vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn inactive_duplicates_do_not_block_active_first_occurrence() {
        let rows = [
            ("core", false),
            ("core", true),
            ("edge", true),
        ];
        assert_eq!(selected_tags(&rows), vec!["core", "edge"]);
    }
}
