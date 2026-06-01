pub fn parse_record(line: &str) -> Result<(&str, u32, bool), &'static str> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 3 {
        return Err("field count");
    }

    let name = parts[0];
    if name.is_empty() {
        return Err("name");
    }

    let qty: u32 = parts[1].parse().map_err(|_| "qty")?;
    let enabled = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("enabled"),
    };

    Ok((name, qty, enabled))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("widget|12|true"), Ok(("widget", 12, true)));
    }

    #[test]
    fn rejects_whitespace_around_name() {
        assert_eq!(parse_record(" widget |12|true"), Err("name"));
    }

    #[test]
    fn rejects_negative_quantity() {
        assert_eq!(parse_record("widget|-2|false"), Err("qty"));
    }

    #[test]
    fn rejects_bad_bool_text() {
        assert_eq!(parse_record("widget|2|yes"), Err("enabled"));
    }
}
