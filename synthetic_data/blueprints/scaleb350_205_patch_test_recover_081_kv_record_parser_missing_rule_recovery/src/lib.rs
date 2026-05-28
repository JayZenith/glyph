pub fn parse_record(line: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();

    for part in line.split(';') {
        if part.is_empty() {
            continue;
        }

        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("missing '=' in segment: {}", part))?;

        if key.is_empty() {
            return Err("empty key".into());
        }
        if !key.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
            return Err(format!("invalid key: {}", key));
        }
        if value.is_empty() {
            return Err(format!("empty value for key: {}", key));
        }

        out.push((key.to_string(), value.to_string()));
    }

    if out.is_empty() {
        return Err("no fields".into());
    }

    let has_id = out.iter().any(|(k, _)| k == "id");
    let has_name = out.iter().any(|(k, _)| k == "name");

    if !has_id || !has_name {
        return Err("missing required field".into());
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        let rec = parse_record("id=42;name=widget;status=ok").unwrap();
        assert_eq!(rec[0], ("id".to_string(), "42".to_string()));
        assert_eq!(rec[1], ("name".to_string(), "widget".to_string()));
        assert_eq!(rec[2], ("status".to_string(), "ok".to_string()));
    }

    #[test]
    fn rejects_duplicate_keys() {
        let err = parse_record("id=1;name=alpha;name=beta").unwrap_err();
        assert!(err.contains("duplicate key"), "got: {err}");
    }

    #[test]
    fn rejects_non_numeric_id() {
        let err = parse_record("id=abc;name=alpha").unwrap_err();
        assert!(err.contains("id must be numeric"), "got: {err}");
    }

    #[test]
    fn rejects_whitespace_in_value() {
        let err = parse_record("id=7;name=bad value").unwrap_err();
        assert!(err.contains("whitespace"), "got: {err}");
    }

    #[test]
    fn rejects_unknown_field() {
        let err = parse_record("id=7;name=alpha;extra=nope").unwrap_err();
        assert!(err.contains("unknown key"), "got: {err}");
    }
}
