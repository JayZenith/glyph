pub fn collect_tags(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .filter_map(|item| {
            let (name, count) = item.split_once(':')?;
            let qty: usize = count.parse().ok()?;
            if qty == 0 {
                return None;
            }
            Some(name.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_tags;

    #[test]
    fn drops_zero_invalid_and_blank_names() {
        let input = ["alpha:2", "beta:0", "oops", ":4", "gamma:x", "  :3"];
        let got = collect_tags(&input);
        assert_eq!(got, vec!["alpha", "alpha"]);
    }

    #[test]
    fn trims_names_and_preserves_input_order() {
        let input = [" red :1", "blue:2", " green:1 "];
        let got = collect_tags(&input);
        assert_eq!(got, vec!["red", "blue", "blue", "green"]);
    }
}
