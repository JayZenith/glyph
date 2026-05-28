pub fn parse_record(line: &str) -> Result<(&str, u16, bool), String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 3 {
        return Err("expected 3 fields".into());
    }

    let name = parts[0];
    if name.is_empty() {
        return Err("empty name".into());
    }

    let qty: u16 = parts[1].parse().map_err(|_| "bad qty".to_string())?;
    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("bad active flag".into()),
    };

    Ok((name, qty, active))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record_with_trimmed_fields() {
        assert_eq!(parse_record(" widget | 12 | true "), Ok(("widget", 12, true)));
    }

    #[test]
    fn rejects_missing_or_extra_fields() {
        assert!(parse_record("a|1").is_err());
        assert!(parse_record("a|1|true|x").is_err());
    }

    #[test]
    fn rejects_empty_name_after_trim() {
        assert!(parse_record("   |2|false").is_err());
    }

    #[test]
    fn rejects_leading_zero_quantity_except_zero() {
        assert_eq!(parse_record("a|0|false"), Ok(("a", 0, false)));
        assert!(parse_record("a|007|true").is_err());
    }

    #[test]
    fn rejects_non_lowercase_boolean_text() {
        assert!(parse_record("a|1|TRUE").is_err());
        assert!(parse_record("a|1|False").is_err());
    }
}
