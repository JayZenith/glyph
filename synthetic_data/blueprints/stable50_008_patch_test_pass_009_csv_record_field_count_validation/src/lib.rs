pub fn parse_record(line: &str) -> Result<(&str, u32, bool), &'static str> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return Err("invalid field count");
    }

    let id = parts[0];
    if id.is_empty() {
        return Err("missing id");
    }

    let qty = parts[1].parse::<u32>().map_err(|_| "invalid qty")?;
    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("invalid active flag"),
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
    fn rejects_too_few_fields() {
        assert_eq!(parse_record("item-7,42"), Err("invalid field count"));
    }

    #[test]
    fn rejects_too_many_fields() {
        assert_eq!(parse_record("item-7,42,true,extra"), Err("invalid field count"));
    }

    #[test]
    fn rejects_invalid_bool() {
        assert_eq!(parse_record("item-7,42,yes"), Err("invalid active flag"));
    }
}
