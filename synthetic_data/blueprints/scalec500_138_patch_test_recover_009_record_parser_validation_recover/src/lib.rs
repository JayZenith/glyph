pub fn parse_record(line: &str) -> Result<(String, u16, bool), String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 3 {
        return Err("expected 3 fields".to_string());
    }

    let name = parts[0].to_string();
    if name.is_empty() {
        return Err("empty name".to_string());
    }

    let qty: u16 = parts[1].parse().map_err(|_| "bad qty".to_string())?;

    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("bad active".to_string()),
    };

    Ok((name, qty, active))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("widget|12|true").unwrap(), ("widget".to_string(), 12, true));
    }

    #[test]
    fn trims_around_fields() {
        assert_eq!(parse_record("  gadget | 7 | false ").unwrap(), ("gadget".to_string(), 7, false));
    }

    #[test]
    fn rejects_extra_delimiters() {
        assert!(parse_record("a|1|true|oops").is_err());
    }

    #[test]
    fn rejects_non_ascii_name() {
        assert_eq!(parse_record("cafeé|2|true").unwrap_err(), "bad name");
    }

    #[test]
    fn rejects_name_with_space() {
        assert_eq!(parse_record("two words|2|true").unwrap_err(), "bad name");
    }

    #[test]
    fn rejects_leading_zero_qty_except_zero() {
        assert_eq!(parse_record("widget|007|true").unwrap_err(), "bad qty");
        assert_eq!(parse_record("widget|0|true").unwrap(), ("widget".to_string(), 0, true));
    }
}
