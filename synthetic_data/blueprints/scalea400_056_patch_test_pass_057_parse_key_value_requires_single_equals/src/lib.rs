pub fn parse_pairs(input: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("invalid line: {line}"));
        };

        if key.is_empty() || value.is_empty() {
            return Err(format!("invalid line: {line}"));
        }

        out.push((key.to_string(), value.to_string()));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_pairs;

    #[test]
    fn parses_valid_pairs_and_skips_blank_lines() {
        let input = "host=localhost\n\nport=8080\nmode=dev";
        let parsed = parse_pairs(input).unwrap();
        assert_eq!(
            parsed,
            vec![
                ("host".to_string(), "localhost".to_string()),
                ("port".to_string(), "8080".to_string()),
                ("mode".to_string(), "dev".to_string()),
            ]
        );
    }

    #[test]
    fn rejects_missing_separator() {
        assert!(parse_pairs("host").is_err());
    }

    #[test]
    fn rejects_empty_key_or_value() {
        assert!(parse_pairs("=localhost").is_err());
        assert!(parse_pairs("host=").is_err());
    }

    #[test]
    fn rejects_multiple_separators() {
        assert!(parse_pairs("path=a=b").is_err());
    }
}
