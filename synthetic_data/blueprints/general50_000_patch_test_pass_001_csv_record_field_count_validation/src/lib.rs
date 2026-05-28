pub fn parse_record(line: &str) -> Result<(&str, u32, bool), String> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return Err("expected 3 fields".to_string());
    }

    let name = parts[0];
    if name.is_empty() {
        return Err("name is empty".to_string());
    }

    let qty = parts[1]
        .parse::<u32>()
        .map_err(|_| "invalid quantity".to_string())?;

    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("invalid active flag".to_string()),
    };

    Ok((name, qty, active))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("apple,12,true"), Ok(("apple", 12, true)));
    }

    #[test]
    fn rejects_missing_field() {
        assert_eq!(parse_record("apple,12"), Err("expected 3 fields".to_string()));
    }

    #[test]
    fn rejects_extra_field() {
        assert_eq!(parse_record("apple,12,true,extra"), Err("expected 3 fields".to_string()));
    }

    #[test]
    fn rejects_invalid_bool() {
        assert_eq!(parse_record("apple,12,yes"), Err("invalid active flag".to_string()));
    }
}
