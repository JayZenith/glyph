pub fn parse_line(line: &str) -> Result<(&str, u8, bool), &'static str> {
    let mut parts = line.split('|');
    let name = parts.next().ok_or("missing name")?;
    let age_str = parts.next().ok_or("missing age")?;
    let active_str = parts.next().ok_or("missing active")?;

    if parts.next().is_some() {
        return Err("too many fields");
    }
    if name.is_empty() {
        return Err("empty name");
    }

    let age: u8 = age_str.parse().map_err(|_| "invalid age")?;
    if age > 120 {
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
    use super::parse_line;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_line("Mina|42|true"), Ok(("Mina", 42, true)));
    }

    #[test]
    fn rejects_extra_fields() {
        assert_eq!(parse_line("Mina|42|true|x"), Err("too many fields"));
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(parse_line("|42|false"), Err("empty name"));
    }

    #[test]
    fn rejects_invalid_age_text() {
        assert_eq!(parse_line("Mina|old|true"), Err("invalid age"));
    }

    #[test]
    fn rejects_age_out_of_range() {
        assert_eq!(parse_line("Mina|121|true"), Err("age out of range"));
    }

    #[test]
    fn rejects_invalid_active_flag() {
        assert_eq!(parse_line("Mina|42|yes"), Err("invalid active"));
    }
}
