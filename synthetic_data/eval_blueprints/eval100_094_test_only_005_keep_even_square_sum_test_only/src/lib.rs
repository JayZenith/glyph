pub fn sum_even_squares(values: &[i32]) -> i32 {
    values
        .iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .sum()
}

pub fn collect_named_active<'a>(items: &'a [(&'a str, bool)]) -> Vec<&'a str> {
    items
        .iter()
        .filter(|(_, active)| *active)
        .map(|(name, _)| *name)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{collect_named_active, sum_even_squares};

    #[test]
    fn sums_only_even_squares() {
        assert_eq!(sum_even_squares(&[1, 2, 3, 4, 5]), 20);
        assert_eq!(sum_even_squares(&[-2, -1, 0, 3]), 4);
    }

    #[test]
    fn collects_only_active_names_in_order() {
        let items = [("alpha", true), ("beta", false), ("gamma", true)];
        assert_eq!(collect_named_active(&items), vec!["alpha", "gamma"]);
    }
}
