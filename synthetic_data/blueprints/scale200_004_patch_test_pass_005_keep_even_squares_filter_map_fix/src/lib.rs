pub fn even_squares(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .filter(|n| *n % 2 != 0)
        .map(|n| n * n)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::even_squares;

    #[test]
    fn keeps_only_even_inputs_and_squares_them() {
        assert_eq!(even_squares(&[1, 2, 3, 4, 5]), vec![4, 16]);
    }

    #[test]
    fn preserves_order_of_matching_values() {
        assert_eq!(even_squares(&[6, 1, 8, 3, 10]), vec![36, 64, 100]);
    }

    #[test]
    fn empty_when_no_even_numbers() {
        assert!(even_squares(&[1, 3, 5]).is_empty());
    }
}
