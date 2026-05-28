pub fn parse_record(line: &str) -> Option<(&str, u32, bool)> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 3 {
        return None;
    }

    let name = parts[0];
    if name.is_empty() {
        return None;
    }

    let qty = parts[1].parse().ok()?;
    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return None,
    };

    Some((name, qty, active))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("widget|12|true"), Some(("widget", 12, true)));
    }

    #[test]
    fn rejects_missing_field() {
        assert_eq!(parse_record("widget|12"), None);
    }

    #[test]
    fn rejects_extra_field() {
        assert_eq!(parse_record("widget|12|true|oops"), None);
    }

    #[test]
    fn rejects_bad_boolean() {
        assert_eq!(parse_record("widget|12|yes"), None);
    }
}
