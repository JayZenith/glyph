pub fn short_even_squares(values: &[i32]) -> Vec<String> {
    values
        .iter()
        .filter(|n| *n % 2 == 0)
        .map(|n| (n * n).to_string())
        .filter(|s| s.len() >= 3)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::short_even_squares;

    #[test]
    fn keeps_only_even_values_and_short_square_strings() {
        let nums = [1, 2, 4, 10, 12, 16];
        assert_eq!(short_even_squares(&nums), vec!["4", "16"]);
    }

    #[test]
    fn ignores_odd_values_before_mapping() {
        let nums = [3, 5, 6, 8];
        assert_eq!(short_even_squares(&nums), vec!["36", "64"]);
    }

    #[test]
    fn returns_empty_when_no_square_strings_are_short_enough() {
        let nums = [20, 22, 24];
        let out: Vec<String> = Vec::new();
        assert_eq!(short_even_squares(&nums), out);
    }
}
