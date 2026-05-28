pub fn collect_labels(items: &[(&str, i32, bool)]) -> Vec<String> {
    items
        .iter()
        .filter(|(_, score, _)| *score >= 10)
        .map(|(label, _, _)| label.trim().to_ascii_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_labels;

    #[test]
    fn keeps_only_active_items_meeting_threshold() {
        let items = [
            ("alpha", 12, true),
            ("beta", 12, false),
            ("gamma", 9, true),
            ("delta", 10, true),
        ];

        assert_eq!(collect_labels(&items), vec!["ALPHA", "DELTA"]);
    }

    #[test]
    fn skips_blank_labels_after_trimming() {
        let items = [
            ("  ", 20, true),
            (" ok ", 15, true),
            ("", 30, true),
        ];

        assert_eq!(collect_labels(&items), vec!["OK"]);
    }

    #[test]
    fn preserves_input_order() {
        let items = [
            (" zed ", 10, true),
            ("amy", 50, true),
            ("bob", 10, false),
            ("cy", 11, true),
        ];

        assert_eq!(collect_labels(&items), vec!["ZED", "AMY", "CY"]);
    }
}
