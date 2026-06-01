pub fn even_squares_over(limit: i32, values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .filter(|sq| *sq < limit)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::even_squares_over;

    #[test]
    fn keeps_even_values_whose_squares_meet_limit() {
        assert_eq!(even_squares_over(16, &[1, 2, 3, 4, 5, 6]), vec![16, 36]);
    }

    #[test]
    fn ignores_odds_and_preserves_input_order() {
        assert_eq!(even_squares_over(20, &[8, 3, 4, 7, 2]), vec![64]);
    }

    #[test]
    fn returns_empty_when_nothing_qualifies() {
        assert_eq!(even_squares_over(100, &[1, 3, 5, 8]), vec![]);
    }
}
