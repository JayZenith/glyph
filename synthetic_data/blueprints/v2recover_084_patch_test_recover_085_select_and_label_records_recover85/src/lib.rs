pub fn selected_labels(items: &[(&str, i32, bool)]) -> Vec<String> {
    items
        .iter()
        .filter(|(_, score, active)| *active || *score > 0)
        .map(|(name, score, _)| format!("{}:{}", name.to_ascii_uppercase(), score))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::selected_labels;

    #[test]
    fn keeps_only_active_positive_scores_and_formats() {
        let items = [
            ("alice", 3, true),
            ("bob", 0, true),
            ("carol", 5, false),
            ("dave", -2, true),
            ("erin", 8, true),
            ("frank", 1, false),
        ];

        assert_eq!(
            selected_labels(&items),
            vec!["ALICE#3", "ERIN#8"]
        );
    }

    #[test]
    fn preserves_input_order_for_selected_items() {
        let items = [
            ("zoe", 2, true),
            ("amy", 7, true),
            ("ian", -1, true),
        ];

        assert_eq!(selected_labels(&items), vec!["ZOE#2", "AMY#7"]);
    }
}
