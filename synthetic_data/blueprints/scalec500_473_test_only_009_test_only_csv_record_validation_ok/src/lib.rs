pub fn parse_record(line: &str) -> Result<(&str, u32, bool), &'static str> {
    let mut parts = line.split(',');
    let name = parts.next().ok_or("missing name")?;
    let age_str = parts.next().ok_or("missing age")?;
    let active_str = parts.next().ok_or("missing active")?;
    if parts.next().is_some() {
        return Err("too many fields");
    }
    if name.is_empty() {
        return Err("empty name");
    }
    let age: u32 = age_str.parse().map_err(|_| "invalid age")?;
    if age > 130 {
        return Err("age out of range");
    }
    let active = match active_str {
        "true" => true,
        "false" => false,
        _ => return Err("invalid active"),
    };
    Ok((name, age, active))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("alice,34,true"), Ok(("alice", 34, true)));
    }

    #[test]
    fn rejects_missing_field() {
        assert_eq!(parse_record("alice,34"), Err("missing active"));
    }

    #[test]
    fn rejects_extra_field() {
        assert_eq!(parse_record("alice,34,true,admin"), Err("too many fields"));
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(parse_record(",34,true"), Err("empty name"));
    }

    #[test]
    fn rejects_invalid_age() {
        assert_eq!(parse_record("alice,xx,true"), Err("invalid age"));
    }

    #[test]
    fn rejects_age_out_of_range() {
        assert_eq!(parse_record("alice,131,true"), Err("age out of range"));
    }

    #[test]
    fn rejects_invalid_active() {
        assert_eq!(parse_record("alice,34,yes"), Err("invalid active"));
    }
}
