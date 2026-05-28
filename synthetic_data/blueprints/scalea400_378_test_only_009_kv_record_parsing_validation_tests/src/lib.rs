pub fn parse_record(line: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    if line.is_empty() {
        return Err("empty record".to_string());
    }
    for field in line.split(';') {
        let (key, value) = field
            .split_once('=')
            .ok_or_else(|| format!("missing '=' in field: {field}"))?;
        if key.is_empty() {
            return Err("empty key".to_string());
        }
        if value.is_empty() {
            return Err(format!("empty value for key: {key}"));
        }
        if !key.bytes().all(|b| b.is_ascii_lowercase() || b == b'_') {
            return Err(format!("invalid key: {key}"));
        }
        out.push((key.to_string(), value.to_string()));
    }
    Ok(out)
}

pub fn validate_required(fields: &[(String, String)], required: &[&str]) -> Result<(), String> {
    for key in required {
        if !fields.iter().any(|(k, _)| k == key) {
            return Err(format!("missing required key: {key}"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let got = parse_record("name=widget;sku=abc_123;status=active").unwrap();
        assert_eq!(
            got,
            vec![
                ("name".to_string(), "widget".to_string()),
                ("sku".to_string(), "abc_123".to_string()),
                ("status".to_string(), "active".to_string()),
            ]
        );
    }

    #[test]
    fn rejects_missing_separator() {
        let err = parse_record("name=widget;broken").unwrap_err();
        assert_eq!(err, "missing '=' in field: broken");
    }

    #[test]
    fn rejects_empty_key() {
        let err = parse_record("=value").unwrap_err();
        assert_eq!(err, "empty key");
    }

    #[test]
    fn rejects_empty_value() {
        let err = parse_record("name=").unwrap_err();
        assert_eq!(err, "empty value for key: name");
    }

    #[test]
    fn rejects_invalid_key_chars() {
        let err = parse_record("Name=value").unwrap_err();
        assert_eq!(err, "invalid key: Name");
    }

    #[test]
    fn validates_required_keys() {
        let fields = parse_record("name=widget;status=active").unwrap();
        assert!(validate_required(&fields, &["name", "status"]).is_ok());
    }

    #[test]
    fn missing_required_key_is_reported() {
        let fields = parse_record("name=widget;status=active").unwrap();
        let err = validate_required(&fields, &["name", "sku"]).unwrap_err();
        assert_eq!(err, "missing required key: sku");
    }
}
