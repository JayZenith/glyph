pub fn parse_record(line: &str) -> Result<(&str, u32, bool), &'static str> {
    let mut parts = line.split('|');
    let name = parts.next().ok_or("missing name")?;
    let count = parts.next().ok_or("missing count")?;
    let enabled = parts.next().ok_or("missing enabled")?;

    if parts.next().is_some() {
        return Err("too many fields");
    }
    if name.is_empty() {
        return Err("empty name");
    }

    let count: u32 = count.parse().map_err(|_| "invalid count")?;
    let enabled = match enabled {
        "true" => true,
        "false" => false,
        _ => return Err("invalid enabled"),
    };

    Ok((name, count, enabled))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("widget|12|true"), Ok(("widget", 12, true)));
    }

    #[test]
    fn rejects_missing_field() {
        assert_eq!(parse_record("widget|12"), Err("missing enabled"));
    }

    #[test]
    fn rejects_extra_field() {
        assert_eq!(parse_record("widget|12|true|extra"), Err("too many fields"));
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(parse_record("|12|false"), Err("empty name"));
    }

    #[test]
    fn rejects_invalid_count() {
        assert_eq!(parse_record("widget|x|false"), Err("invalid count"));
    }

    #[test]
    fn rejects_negative_count() {
        assert_eq!(parse_record("widget|-3|false"), Err("invalid count"));
    }

    #[test]
    fn rejects_invalid_enabled() {
        assert_eq!(parse_record("widget|3|yes"), Err("invalid enabled"));
    }
}
