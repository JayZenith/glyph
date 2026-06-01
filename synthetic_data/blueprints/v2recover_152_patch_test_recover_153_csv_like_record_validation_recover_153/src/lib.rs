pub fn parse_record(line: &str) -> Result<(String, u32, bool), String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 3 {
        return Err("expected 3 fields".to_string());
    }

    let name = parts[0].to_string();
    if name.is_empty() {
        return Err("empty name".to_string());
    }
    if !name.chars().all(|c| c.is_ascii_alphabetic() || c == ' ') {
        return Err("invalid name".to_string());
    }

    let age: u32 = parts[1].parse().map_err(|_| "invalid age".to_string())?;
    if age > 130 {
        return Err("age out of range".to_string());
    }

    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("invalid active flag".to_string()),
    };

    Ok((name, age, active))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record_with_trimmed_fields() {
        let got = parse_record("  Ada Lovelace | 36 | true ").unwrap();
        assert_eq!(got, ("Ada Lovelace".to_string(), 36, true));
    }

    #[test]
    fn rejects_missing_name_after_trim() {
        assert_eq!(parse_record("   |42|false").unwrap_err(), "empty name");
    }

    #[test]
    fn rejects_leading_zero_age() {
        assert_eq!(parse_record("Bob|007|true").unwrap_err(), "invalid age");
    }

    #[test]
    fn rejects_non_alpha_name() {
        assert_eq!(parse_record("Eve2|20|false").unwrap_err(), "invalid name");
    }
}
