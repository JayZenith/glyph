pub fn parse_record(line: &str) -> Result<(&str, u32, bool), &'static str> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 3 {
        return Err("field count");
    }

    let id = parts[0];
    if id.is_empty() || !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err("id");
    }

    let qty: u32 = parts[1].parse().map_err(|_| "qty")?;
    if qty == 0 {
        return Err("qty");
    }

    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("active"),
    };

    Ok((id, qty, active))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("item-7,42,true"), Ok(("item-7", 42, true)));
    }

    #[test]
    fn rejects_wrong_field_count() {
        assert_eq!(parse_record("item-7,42"), Err("field count"));
        assert_eq!(parse_record("a,1,true,extra"), Err("field count"));
    }

    #[test]
    fn rejects_bad_id() {
        assert_eq!(parse_record(",4,true"), Err("id"));
        assert_eq!(parse_record("bad id,4,true"), Err("id"));
    }

    #[test]
    fn rejects_bad_quantity() {
        assert_eq!(parse_record("item-7,0,true"), Err("qty"));
        assert_eq!(parse_record("item-7,nope,true"), Err("qty"));
    }

    #[test]
    fn rejects_bad_boolean() {
        assert_eq!(parse_record("item-7,4,yes"), Err("active"));
    }
}
