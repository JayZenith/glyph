pub fn square_evens_above(values: &[i32], min: i32) -> Vec<i32> {
    values
        .iter()
        .copied()
        .filter(|n| *n > min)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .collect()
}

pub fn sum_tag_lengths(tags: &[&str]) -> usize {
    tags.iter()
        .filter(|tag| !tag.is_empty())
        .map(|tag| tag.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_even_numbers_strictly_above_min_and_squares_them() {
        let input = [1, 2, 3, 4, 5, 6, 8];
        assert_eq!(square_evens_above(&input, 4), vec![36, 64]);
    }

    #[test]
    fn returns_empty_when_nothing_matches() {
        let input = [0, 1, 2, 3];
        assert_eq!(square_evens_above(&input, 10), Vec::<i32>::new());
    }

    #[test]
    fn sums_only_non_empty_tag_lengths() {
        let tags = ["red", "", "blue", "go"];
        assert_eq!(sum_tag_lengths(&tags), 9);
    }
}
