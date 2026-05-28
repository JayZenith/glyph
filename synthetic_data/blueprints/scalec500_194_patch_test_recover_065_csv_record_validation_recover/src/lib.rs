pub fn parse_record(line: &str) -> Result<(&str, u32, bool), String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 3 {
        return Err("expected 3 fields".into());
    }

    let name = parts[0];
    if name.is_empty() {
        return Err("name missing".into());
    }

    let qty: u32 = parts[1].parse().map_err(|_| "bad quantity")?;
    if qty == 0 {
        return Err("quantity must be positive".into());
    }

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
    fn parses_valid_line() {
        assert_eq!(parse_record("widget|12|true").unwrap(), ("widget", 12, true));
    }

    #[test]
    fn rejects_wrong_field_count() {
        assert!(parse_record("widget|12|true|extra").is_err());
        assert!(parse_record("widget|12").is_err());
    }

    #[test]
    fn rejects_blank_name_after_trimming() {
        assert!(parse_record("   |4|false").is_err());
    }

    #[test]
    fn parses_trimmed_fields_and_case_insensitive_bool() {
        assert_eq!(parse_record("  gizmo  |7|FALSE ").unwrap(), ("gizmo", 7, false));
    }

    #[test]
    fn rejects_non_numeric_or_zero_quantity() {
        assert!(parse_record("gizmo|abc|true").is_err());
        assert!(parse_record("gizmo|0|true").is_err());
    }
}
