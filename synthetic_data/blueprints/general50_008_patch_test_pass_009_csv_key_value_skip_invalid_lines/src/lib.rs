pub fn parse_pairs(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .filter_map(|line| {
            let (key, value) = line.split_once('=')?;
            Some((key.trim().to_string(), value.trim().to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_pairs;

    #[test]
    fn keeps_valid_pairs_and_skips_bad_lines() {
        let input = "host = localhost\ninvalid\n=missing_key\nport=8080\nmode=\nthreads = 4";
        let pairs = parse_pairs(input);
        assert_eq!(
            pairs,
            vec![
                ("host".to_string(), "localhost".to_string()),
                ("port".to_string(), "8080".to_string()),
                ("threads".to_string(), "4".to_string()),
            ]
        );
    }

    #[test]
    fn trims_around_key_and_value() {
        let input = " alpha = one \n beta= two";
        let pairs = parse_pairs(input);
        assert_eq!(
            pairs,
            vec![
                ("alpha".to_string(), "one".to_string()),
                ("beta".to_string(), "two".to_string()),
            ]
        );
    }
}
