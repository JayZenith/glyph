pub fn parse_record(line: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    for part in line.split(';') {
        if part.is_empty() {
            continue;
        }
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("missing '=' in segment: {part}"))?;
        if key.is_empty() {
            return Err("empty key".into());
        }
        if value.is_empty() {
            return Err(format!("empty value for key: {key}"));
        }
        if !key.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
            return Err(format!("invalid key: {key}"));
        }
        if !value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(format!("invalid value for key: {key}"));
        }
        out.push((key.to_string(), value.to_string()));
    }
    if out.is_empty() {
        return Err("no fields".into());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        let parsed = parse_record("user_id=abc123;role=admin").unwrap();
        assert_eq!(
            parsed,
            vec![
                ("user_id".to_string(), "abc123".to_string()),
                ("role".to_string(), "admin".to_string())
            ]
        );
    }

    #[test]
    fn rejects_missing_equals() {
        let err = parse_record("user_id=abc;role").unwrap_err();
        assert!(err.contains("missing '='"));
    }

    #[test]
    fn rejects_empty_key() {
        assert_eq!(parse_record("=x").unwrap_err(), "empty key");
    }

    #[test]
    fn rejects_empty_value() {
        assert_eq!(
            parse_record("user_id=").unwrap_err(),
            "empty value for key: user_id"
        );
    }

    #[test]
    fn rejects_invalid_key_chars() {
        assert_eq!(parse_record("User=abc").unwrap_err(), "invalid key: User");
    }

    #[test]
    fn rejects_invalid_value_chars() {
        assert_eq!(
            parse_record("user_id=a-b").unwrap_err(),
            "invalid value for key: user_id"
        );
    }

    #[test]
    fn ignores_trailing_separator() {
        let parsed = parse_record("user_id=abc123;").unwrap();
        assert_eq!(parsed, vec![("user_id".to_string(), "abc123".to_string())]);
    }

    #[test]
    fn rejects_empty_input() {
        assert_eq!(parse_record("").unwrap_err(), "no fields");
    }
}
