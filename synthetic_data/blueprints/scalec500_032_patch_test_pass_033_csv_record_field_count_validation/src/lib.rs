pub fn parse_record(line: &str) -> Result<(&str, u32, bool), String> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return Err("expected exactly 3 fields".to_string());
    }

    let id = parts[0];
    if id.is_empty() {
        return Err("missing id".to_string());
    }

    let qty = parts[1]
        .parse::<u32>()
        .map_err(|_| "invalid quantity".to_string())?;

    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("invalid active flag".to_string()),
    };

    Ok((id, qty, active))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("item-7,12,true"), Ok(("item-7", 12, true)));
    }

    #[test]
    fn rejects_too_few_fields() {
        assert!(parse_record("item-7,12").is_err());
    }

    #[test]
    fn rejects_too_many_fields() {
        assert!(parse_record("item-7,12,true,extra").is_err());
    }

    #[test]
    fn rejects_invalid_bool() {
        assert!(parse_record("item-7,12,yes").is_err());
    }
}
