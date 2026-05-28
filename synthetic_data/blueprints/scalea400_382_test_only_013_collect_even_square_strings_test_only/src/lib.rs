pub fn even_square_labels(values: &[i32]) -> Vec<String> {
    values
        .iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .map(|n| format!("{}:{}", n, n * n))
        .collect()
}

pub fn sum_large_even_squares(values: &[i32], min_square: i32) -> i32 {
    values
        .iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .filter(|sq| *sq >= min_square)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{even_square_labels, sum_large_even_squares};

    #[test]
    fn labels_only_even_numbers_in_order() {
        let input = [3, 4, -2, 5, 0];
        let got = even_square_labels(&input);
        assert_eq!(got, vec!["4:16", "-2:4", "0:0"]);
    }

    #[test]
    fn sums_only_even_squares_meeting_threshold() {
        let input = [1, 2, 3, 4, 6];
        assert_eq!(sum_large_even_squares(&input, 10), 52);
    }

    #[test]
    fn empty_when_no_even_values_match() {
        let input = [1, 3, 5];
        assert!(even_square_labels(&input).is_empty());
        assert_eq!(sum_large_even_squares(&input, 1), 0);
    }
}
