pub fn positive_ids(items: &[&str]) -> Vec<u32> {
    items
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .filter(|n| *n >= 0)
        .map(|n| n as u32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::positive_ids;

    #[test]
    fn keeps_only_positive_numbers() {
        let input = ["12", "x", "0", "-3", "7"];
        assert_eq!(positive_ids(&input), vec![12, 7]);
    }

    #[test]
    fn skips_invalid_and_negative_values() {
        let input = ["9", "-1", "abc", "15"];
        assert_eq!(positive_ids(&input), vec![9, 15]);
    }
}
