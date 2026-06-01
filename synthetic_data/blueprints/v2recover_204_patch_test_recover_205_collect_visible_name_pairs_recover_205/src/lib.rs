pub fn collect_visible_name_pairs(rows: &[(&str, Option<&str>, bool)]) -> Vec<String> {
    rows.iter()
        .filter(|(_, _, active)| *active)
        .filter_map(|(id, name, _)| name.map(|name| format!("{}={}", id, name.trim())))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_visible_name_pairs;

    #[test]
    fn keeps_only_active_rows_with_non_blank_names() {
        let rows = [
            ("a1", Some("  Alice  "), true),
            ("b2", Some("   "), true),
            ("c3", None, true),
            ("d4", Some("Dora"), false),
            ("e5", Some(" Eve "), true),
        ];

        assert_eq!(
            collect_visible_name_pairs(&rows),
            vec!["a1=Alice", "e5=Eve"]
        );
    }

    #[test]
    fn preserves_input_order_after_filtering() {
        let rows = [
            ("z9", Some(" Zoe "), true),
            ("a0", Some("Ann"), true),
            ("m3", Some("   "), true),
            ("x1", Some("Xena"), false),
            ("b7", Some(" Ben"), true),
        ];

        assert_eq!(
            collect_visible_name_pairs(&rows),
            vec!["z9=Zoe", "a0=Ann", "b7=Ben"]
        );
    }
}
