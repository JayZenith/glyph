pub fn collect_tagged_numbers(input: &str) -> Vec<i32> {
    input
        .split(',')
        .filter_map(|part| part.split_once(':'))
        .filter(|(tag, _)| *tag == "keep")
        .filter_map(|(_, value)| value.parse::<i32>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::collect_tagged_numbers;

    #[test]
    fn keeps_only_keep_tagged_values() {
        assert_eq!(collect_tagged_numbers("keep:1,drop:9,keep:3"), vec![1, 3]);
    }

    #[test]
    fn ignores_empty_values_and_bad_numbers() {
        assert_eq!(collect_tagged_numbers("keep:,keep:7,keep:abc,drop:4,keep:2"), vec![7, 2]);
    }

    #[test]
    fn trims_parts_and_supports_signed_numbers() {
        assert_eq!(collect_tagged_numbers(" keep : -2 , keep : 5 , drop : -1 "), vec![-2, 5]);
    }
}
