pub fn numeric_prefixes(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .filter_map(|item| {
            let prefix: String = item.chars().take_while(|c| c.is_ascii_digit()).collect();
            if prefix.is_empty() || prefix.len() == item.len() {
                None
            } else {
                Some(prefix)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::numeric_prefixes;

    #[test]
    fn keeps_only_leading_digit_runs() {
        let items = ["12ab", "x9", "007bond", "42", "", "5ive"];
        assert_eq!(numeric_prefixes(&items), vec!["12", "007", "42", "5"]);
    }

    #[test]
    fn skips_items_without_leading_digits() {
        let items = ["abc", "", "-12", "z99"];
        let actual: Vec<String> = numeric_prefixes(&items);
        assert!(actual.is_empty());
    }
}
