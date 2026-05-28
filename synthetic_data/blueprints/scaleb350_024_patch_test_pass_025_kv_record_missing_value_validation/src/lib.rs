pub fn parse_record(line: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    for part in line.split(';') {
        if part.is_empty() {
            continue;
        }
        let Some((key, value)) = part.split_once('=') else {
            return Err(format!("missing '=' in {part}"));
        };
        if key.is_empty() {
            return Err("empty key".to_string());
        }
        if key.chars().any(|c| !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != '_') {
            return Err(format!("invalid key {key}"));
        }
        out.push((key.to_string(), value.to_string()));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_pairs() {
        let got = parse_record("user_id=42;mode=fast").unwrap();
        assert_eq!(
            got,
            vec![
                ("user_id".to_string(), "42".to_string()),
                ("mode".to_string(), "fast".to_string()),
            ]
        );
    }

    #[test]
    fn rejects_missing_equals() {
        assert!(parse_record("user_id=42;broken").is_err());
    }

    #[test]
    fn rejects_empty_key() {
        assert!(parse_record("=42").is_err());
    }

    #[test]
    fn rejects_empty_value() {
        assert!(parse_record("user_id=").is_err());
    }

    #[test]
    fn rejects_invalid_key_chars() {
        assert!(parse_record("user-id=42").is_err());
    }
}
