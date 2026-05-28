pub fn parse_record(line: &str) -> Result<(&str, u32, bool), &'static str> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return Err("missing fields");
    }

    let name = parts[0];
    if name.is_empty() {
        return Err("empty name");
    }

    let count = parts[1].parse::<u32>().map_err(|_| "invalid count")?;
    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("invalid active flag"),
    };

    Ok((name, count, active))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("widget,12,true"), Ok(("widget", 12, true)));
    }

    #[test]
    fn rejects_missing_fields() {
        assert_eq!(parse_record("widget,12"), Err("missing fields"));
    }

    #[test]
    fn rejects_extra_fields() {
        assert_eq!(parse_record("widget,12,true,extra"), Err("missing fields"));
    }

    #[test]
    fn rejects_invalid_boolean() {
        assert_eq!(parse_record("widget,12,yes"), Err("invalid active flag"));
    }
}
