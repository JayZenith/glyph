pub fn collect_tags(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| {
            let (active, tag) = line.split_once(':')?;
            if active != "on" {
                return None;
            }
            let cleaned = tag.trim();
            if cleaned.is_empty() {
                None
            } else {
                Some(cleaned.to_string())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_tags;

    #[test]
    fn keeps_only_active_normalized_unique_tags() {
        let input = [
            "on: Alpha ",
            "off:Beta",
            "on:alpha",
            "on: MiXed ",
            "on:",
            "on: MIXED",
            "on: gamma",
            "ON:delta",
        ];

        assert_eq!(
            collect_tags(&input),
            vec!["alpha", "mixed", "gamma"]
        );
    }

    #[test]
    fn skips_missing_separator_and_preserves_first_seen_order() {
        let input = [
            "on:red",
            "broken-entry",
            "on:blue",
            "on:RED",
            "off:green",
            "on: blue ",
            "on:green",
        ];

        assert_eq!(collect_tags(&input), vec!["red", "blue", "green"]);
    }
}
