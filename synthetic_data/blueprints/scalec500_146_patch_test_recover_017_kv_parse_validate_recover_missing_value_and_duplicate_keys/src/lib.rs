use std::collections::HashMap;

pub fn parse_kv(input: &str) -> Result<HashMap<String, String>, String> {
    let mut out = HashMap::new();

    for (idx, raw) in input.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("line {}: missing '='", idx + 1))?;

        let key = key.trim();
        let value = value.trim();

        if key.is_empty() {
            return Err(format!("line {}: empty key", idx + 1));
        }

        out.insert(key.to_string(), value.to_string());
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_kv;

    #[test]
    fn parses_basic_pairs_and_ignores_comments() {
        let cfg = parse_kv("\n# config\nhost = localhost\nport=8080\n\n").unwrap();
        assert_eq!(cfg.get("host").map(String::as_str), Some("localhost"));
        assert_eq!(cfg.get("port").map(String::as_str), Some("8080"));
    }

    #[test]
    fn rejects_empty_key() {
        let err = parse_kv("=value").unwrap_err();
        assert!(err.contains("empty key"));
    }

    #[test]
    fn rejects_missing_separator() {
        let err = parse_kv("host localhost").unwrap_err();
        assert!(err.contains("missing '='"));
    }

    #[test]
    fn rejects_empty_value() {
        let err = parse_kv("host=").unwrap_err();
        assert!(err.contains("empty value"));
    }

    #[test]
    fn rejects_duplicate_keys() {
        let err = parse_kv("host=one\nhost=two").unwrap_err();
        assert!(err.contains("duplicate key"));
    }
}
