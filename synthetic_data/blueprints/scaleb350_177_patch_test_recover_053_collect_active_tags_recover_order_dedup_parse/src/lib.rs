pub fn collect_active_tags(rows: &[&str]) -> Vec<String> {
    rows.iter()
        .filter_map(|row| {
            let mut parts = row.split('|');
            let enabled = parts.next()?;
            let raw_tags = parts.next()?;

            if enabled != "on" {
                return None;
            }

            Some(raw_tags)
        })
        .flat_map(|raw| raw.split(','))
        .map(|tag| tag.trim().to_ascii_lowercase())
        .filter(|tag| !tag.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_active_tags;

    #[test]
    fn keeps_first_seen_order_and_dedups_case_insensitively() {
        let rows = [
            "on|Blue, red, blue",
            "off|green,red",
            "on|RED, yellow",
            "on| blue ,Green ",
        ];

        assert_eq!(
            collect_active_tags(&rows),
            vec!["blue", "red", "yellow", "green"]
        );
    }

    #[test]
    fn ignores_rows_without_exactly_two_fields_and_blank_tags() {
        let rows = [
            "on|alpha,,beta",
            "on-only",
            "on|",
            "on| gamma |extra",
            "on| beta ,  , delta ",
        ];

        assert_eq!(
            collect_active_tags(&rows),
            vec!["alpha", "beta", "delta"]
        );
    }

    #[test]
    fn only_accepts_exact_on_flag() {
        let rows = [
            "ON|upper",
            "on |spaced-flag",
            "off|disabled",
            "on|live",
        ];

        assert_eq!(collect_active_tags(&rows), vec!["live"]);
    }
}
