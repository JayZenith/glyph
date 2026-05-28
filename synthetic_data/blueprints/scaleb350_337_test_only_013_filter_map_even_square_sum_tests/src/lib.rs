pub fn even_square_sum(values: &[i32]) -> i32 {
    values
        .iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .sum()
}

pub fn collect_short_codes(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.len() <= 3 && !trimmed.is_empty() {
                Some(trimmed.to_ascii_uppercase())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{collect_short_codes, even_square_sum};

    #[test]
    fn sums_only_even_squares() {
        assert_eq!(even_square_sum(&[1, 2, 3, 4, 5]), 20);
        assert_eq!(even_square_sum(&[-2, -3, 6]), 40);
    }

    #[test]
    fn collects_trimmed_short_codes_uppercase() {
        let items = [" ab", "tool", "xy", "", " Q7 ", "longer"];
        assert_eq!(
            collect_short_codes(&items),
            vec!["AB".to_string(), "XY".to_string(), "Q7".to_string()]
        );
    }
}
