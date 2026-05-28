pub fn collect_even_values(items: &[&str]) -> Vec<i32> {
    items
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .filter(|n| n % 2 != 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_even_values;

    #[test]
    fn keeps_only_even_parsed_numbers() {
        let input = ["1", "2", "x", "8", "-3", "0", "ten", "14"];
        assert_eq!(collect_even_values(&input), vec![2, 8, 0, 14]);
    }

    #[test]
    fn returns_empty_when_no_even_numbers_parse() {
        let input = ["a", "7", "-5", "odd"];
        assert!(collect_even_values(&input).is_empty());
    }
}
