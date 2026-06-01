use std::collections::HashMap;

pub fn parse_record(input: &str) -> Result<HashMap<String, String>, String> {
    let mut out = HashMap::new();

    for part in input.split(';') {
        let piece = part.trim();
        if piece.is_empty() {
            continue;
        }

        let Some((k, v)) = piece.split_once(':') else {
            return Err("missing colon".into());
        };

        let key = k.trim();
        let value = v.trim();

        if key.is_empty() {
            return Err("empty key".into());
        }

        out.insert(key.to_string(), value.to_string());
    }

    if !out.contains_key("id") || !out.contains_key("name") {
        return Err("missing required field".into());
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record_with_spaces() {
        let rec = parse_record(" id : 42 ; name : Ada Lovelace ; city : London ").unwrap();
        assert_eq!(rec.get("id").map(String::as_str), Some("42"));
        assert_eq!(rec.get("name").map(String::as_str), Some("Ada Lovelace"));
        assert_eq!(rec.get("city").map(String::as_str), Some("London"));
    }

    #[test]
    fn rejects_empty_value_for_required_fields() {
        assert!(parse_record("id:42;name:").is_err());
        assert!(parse_record("id:;name:Ada").is_err());
    }

    #[test]
    fn rejects_duplicate_keys() {
        assert!(parse_record("id:1;name:Ada;name:Grace").is_err());
    }

    #[test]
    fn rejects_malformed_segments() {
        assert!(parse_record("id:1;broken;name:Ada").is_err());
        assert!(parse_record(":1;name:Ada").is_err());
    }
}
