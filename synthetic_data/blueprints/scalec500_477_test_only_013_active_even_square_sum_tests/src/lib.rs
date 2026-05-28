pub fn sum_active_even_squares(values: &[(bool, i32)]) -> i32 {
    values
        .iter()
        .filter(|(active, n)| *active && *n % 2 == 0)
        .map(|(_, n)| n * n)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::sum_active_even_squares;

    #[test]
    fn sums_only_active_even_values() {
        let items = [(true, 2), (false, 4), (true, 3), (true, 6)];
        assert_eq!(sum_active_even_squares(&items), 40);
    }

    #[test]
    fn ignores_inactive_and_odd_values() {
        let items = [(false, 8), (true, 5), (false, 1)];
        assert_eq!(sum_active_even_squares(&items), 0);
    }

    #[test]
    fn handles_negative_even_values() {
        let items = [(true, -4), (true, -1), (true, 2)];
        assert_eq!(sum_active_even_squares(&items), 20);
    }
}
