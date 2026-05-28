pub fn selected_scores(items: &[(&str, bool, i32)]) -> Vec<i32> {
    items
        .iter()
        .filter(|(_, active, score)| *active || *score % 2 == 0)
        .map(|(_, _, score)| *score)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::selected_scores;

    #[test]
    fn keeps_only_active_items_with_even_scores() {
        let items = [
            ("a", true, 4),
            ("b", true, 5),
            ("c", false, 8),
            ("d", false, 3),
            ("e", true, 10),
        ];

        assert_eq!(selected_scores(&items), vec![4, 10]);
    }

    #[test]
    fn returns_empty_when_no_item_matches_both_conditions() {
        let items = [("x", true, 1), ("y", false, 2), ("z", false, 7)];
        assert_eq!(selected_scores(&items), Vec::<i32>::new());
    }
}
