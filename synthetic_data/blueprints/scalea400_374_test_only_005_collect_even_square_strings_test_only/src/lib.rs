pub fn even_square_strings(values: &[i32]) -> Vec<String> {
    values
        .iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .filter(|sq| *sq >= 16)
        .map(|sq| format!("sq={sq}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::even_square_strings;

    #[test]
    fn keeps_only_even_values_and_formats_large_squares() {
        let input = [1, 2, 3, 4, 6];
        assert_eq!(
            even_square_strings(&input),
            vec!["sq=16".to_string(), "sq=36".to_string()]
        );
    }

    #[test]
    fn returns_empty_when_no_even_square_meets_threshold() {
        let input = [1, 2, 3];
        let out: Vec<String> = Vec::new();
        assert_eq!(even_square_strings(&input), out);
    }

    #[test]
    fn handles_negative_even_inputs_by_squaring_them() {
        let input = [-5, -4, -2, 0];
        assert_eq!(even_square_strings(&input), vec!["sq=16".to_string()]);
    }
}
