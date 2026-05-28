pub fn parse_record(input: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();

    for raw in input.split(';') {
        let part = raw.trim();
        if part.is_empty() {
            continue;
        }

        let Some((key_raw, value_raw)) = part.split_once('=') else {
            return Err(format!("missing '=' in segment: {}", part));
        };

        let key = key_raw.trim();
        let value = value_raw.trim();

        if key.is_empty() {
            return Err("empty key".into());
        }

        if !key.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
            return Err(format!("invalid key: {}", key));
        }

        if value.is_empty() {
            continue;
        }

        out.push((key.to_string(), value.to_string()));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_basic_pairs_and_trims() {
        let got = parse_record(" user = alice ; role= admin ; active = yes ").unwrap();
        assert_eq!(
            got,
            vec![
                ("user".to_string(), "alice".to_string()),
                ("role".to_string(), "admin".to_string()),
                ("active".to_string(), "yes".to_string()),
            ]
        );
    }

    #[test]
    fn rejects_missing_equals() {
        let err = parse_record("user=alice;broken;role=admin").unwrap_err();
        assert!(err.contains("missing '='"));
    }

    #[test]
    fn rejects_invalid_keys() {
        let err = parse_record("User=alice").unwrap_err();
        assert!(err.contains("invalid key"));
    }

    #[test]
    fn rejects_empty_values() {
        let err = parse_record("user=alice;role=").unwrap_err();
        assert_eq!(err, "empty value for key: role");
    }

    #[test]
    fn rejects_duplicate_keys() {
        let err = parse_record("user=alice;role=admin;user=bob").unwrap_err();
        assert_eq!(err, "duplicate key: user");
    }
}
