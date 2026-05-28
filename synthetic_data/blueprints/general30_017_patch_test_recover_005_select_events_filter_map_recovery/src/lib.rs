pub fn select_event_codes(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| {
            let mut parts = line.split('|');
            let active = parts.next()?;
            let code = parts.next()?;
            let count = parts.next()?;

            if active != "on" {
                return None;
            }

            let qty: u32 = count.parse().ok()?;
            if qty == 0 {
                return None;
            }

            Some(code.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::select_event_codes;

    #[test]
    fn keeps_only_on_rows_with_positive_count_and_uppercase_code() {
        let lines = [
            "on|ALPHA|2",
            "off|BETA|3",
            "on|gamma|5",
            "on|DELTA|0",
            "on|EPSILON|7|extra",
            "on|Z9|1",
        ];

        assert_eq!(select_event_codes(&lines), vec!["ALPHA", "Z9"]);
    }

    #[test]
    fn ignores_bad_counts_and_wrong_field_count() {
        let lines = [
            "on|OK|1",
            "on|MISS",
            "on|NOPE|x",
            "on|WIDE|4|tail",
            "on|FINE|3",
        ];

        assert_eq!(select_event_codes(&lines), vec!["OK", "FINE"]);
    }
}
