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
    fn keeps_only_even_numbers_before_squaring() {
        assert_eq!(even_squares(&[1, 2, 3, 4, 5]), vec![4, 16]);
    }

    #[test]
    fn preserves_input_order_for_selected_items() {
        assert_eq!(even_squares(&[-2, 7, 6, 1]), vec![4, 36]);
    }

    #[test]
    fn empty_when_no_even_values_exist() {
        assert!(even_squares(&[1, 3, 5]).is_empty());
    }
}
