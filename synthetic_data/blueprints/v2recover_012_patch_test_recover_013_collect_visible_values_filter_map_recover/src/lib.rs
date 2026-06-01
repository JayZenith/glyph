pub fn collect_visible_values(items: &[&str]) -> Vec<i32> {
    items
        .iter()
        .filter_map(|item| {
            let (name, value) = item.split_once('=')?;
            if name.starts_with('#') {
                return None;
            }
            value.parse::<i32>().ok()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_visible_values;

    #[test]
    fn skips_hidden_invalid_and_negative_values() {
        let items = ["apples=3", "#draft=9", "oranges=-2", "bad=x", "pears=5"];
        assert_eq!(collect_visible_values(&items), vec![3, 5]);
    }

    #[test]
    fn ignores_blank_names_and_whitespace_around_numbers() {
        let items = ["=7", " kiwi = 4 ", "melon=0", "#skip = 8", "plum= 2"];
        assert_eq!(collect_visible_values(&items), vec![4, 2]);
    }
}
