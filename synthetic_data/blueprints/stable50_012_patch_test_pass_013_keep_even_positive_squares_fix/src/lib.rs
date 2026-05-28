pub fn even_positive_squares(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .copied()
        .filter(|n| *n > 0)
        .map(|n| n * n)
        .filter(|n| n % 2 == 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::even_positive_squares;

    #[test]
    fn keeps_only_even_positive_numbers_and_squares_them() {
        assert_eq!(even_positive_squares(&[-3, -2, 0, 1, 2, 3, 4]), vec![4, 16]);
    }

    #[test]
    fn preserves_input_order_after_filtering() {
        assert_eq!(even_positive_squares(&[6, 5, 2, 7, 8]), vec![36, 4, 64]);
    }

    #[test]
    fn returns_empty_when_no_positive_evens_exist() {
        assert!(even_positive_squares(&[-5, -1, 0, 3, 9]).is_empty());
    }
}
