pub fn collect_tags(lines: &[&str]) -> Vec<String> {
    let mut tags: Vec<String> = lines
        .iter()
        .filter_map(|line| line.split_once(':').map(|(_, rest)| rest))
        .flat_map(|rest| rest.split(','))
        .map(|tag| tag.trim().to_ascii_lowercase())
        .collect();
    tags.sort();
    tags
}

#[cfg(test)]
mod tests {
    use super::collect_tags;

    #[test]
    fn collects_normalized_tags_in_order() {
        let input = [
            "item1: Red, Blue",
            "item2: green,blue",
            "bad line",
            "item3: GREEN , red",
        ];
        assert_eq!(
            collect_tags(&input),
            vec!["blue", "green", "red"]
        );
    }

    #[test]
    fn skips_blank_entries_and_missing_payloads() {
        let input = [
            "item1: alpha, , beta ,",
            "item2:",
            "item3:   ",
            "item4: beta",
        ];
        assert_eq!(collect_tags(&input), vec!["alpha", "beta"]);
    }
}
