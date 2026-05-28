pub fn collect_even_numbers(words: &[&str]) -> Vec<i32> {
    words
        .iter()
        .filter_map(|w| w.parse::<i32>().ok())
        .filter(|n| n % 2 == 1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_even_numbers;

    #[test]
    fn keeps_only_even_parsed_values() {
        let input = ["10", "x", "7", "-4", "0", "abc", "13"];
        assert_eq!(collect_even_numbers(&input), vec![10, -4, 0]);
    }

    #[test]
    fn returns_empty_when_no_even_numbers() {
        let input = ["1", "3", "five"];
        assert_eq!(collect_even_numbers(&input), Vec::<i32>::new());
    }
}
