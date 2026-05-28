pub fn parse_record(line: &str) -> Result<(&str, u32, bool), String> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return Err("expected 3 fields".into());
    }

    let name = parts[0];
    if name.is_empty() {
        return Err("empty name".into());
    }

    let qty = parts[1]
        .parse::<u32>()
        .map_err(|_| "invalid quantity".to_string())?;

    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("invalid active flag".into()),
    };

    Ok((name, qty, active))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("apples,12,true"), Ok(("apples", 12, true)));
    }

    #[test]
    fn rejects_missing_field() {
        assert!(parse_record("apples,12").is_err());
    }

    #[test]
    fn rejects_extra_field() {
        assert!(parse_record("apples,12,true,extra").is_err());
    }

    #[test]
    fn rejects_bad_bool() {
        assert!(parse_record("apples,12,yes").is_err());
    }
}
