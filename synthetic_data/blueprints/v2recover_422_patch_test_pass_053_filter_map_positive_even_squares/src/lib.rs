pub fn positive_even_squares(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .filter(|&&n| n > 0)
        .map(|&n| n * n)
        .filter(|n| n % 2 == 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::positive_even_squares;

    #[test]
    fn keeps_only_positive_even_inputs_and_squares_them() {
        assert_eq!(positive_even_squares(&[-4, -2, 0, 1, 2, 3, 4]), vec![4, 16]);
    }

    #[test]
    fn preserves_input_order() {
        assert_eq!(positive_even_squares(&[6, 1, 8, 5, 2]), vec![36, 64, 4]);
    }

    #[test]
    fn handles_empty_and_all_filtered_out() {
        assert!(positive_even_squares(&[]).is_empty());
        assert!(positive_even_squares(&[-3, -1, 0, 5, 7]).is_empty());
    }
}
