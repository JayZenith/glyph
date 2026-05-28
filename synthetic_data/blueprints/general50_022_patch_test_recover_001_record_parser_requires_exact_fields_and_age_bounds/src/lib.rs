pub fn parse_record(line: &str) -> Result<(&str, u8, bool), &'static str> {
    let parts: Vec<&str> = line.split(';').collect();
    if parts.len() < 3 {
        return Err("field count");
    }

    let name = parts[0];
    if name.is_empty() {
        return Err("name");
    }

    let age: u8 = parts[1].parse().map_err(|_| "age")?;

    let active = match parts[2] {
        "true" | "yes" | "1" => true,
        "false" | "no" | "0" => false,
        _ => return Err("active"),
    };

    Ok((name, age, active))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_records() {
        assert_eq!(parse_record("Alice;34;yes"), Ok(("Alice", 34, true)));
        assert_eq!(parse_record("Bob;0;false"), Ok(("Bob", 0, false)));
    }

    #[test]
    fn rejects_wrong_field_count() {
        assert_eq!(parse_record("Alice;34"), Err("field count"));
        assert_eq!(parse_record("Alice;34;yes;extra"), Err("field count"));
    }

    #[test]
    fn rejects_blank_name() {
        assert_eq!(parse_record(";34;yes"), Err("name"));
    }

    #[test]
    fn rejects_age_out_of_range() {
        assert_eq!(parse_record("Alice;121;yes"), Err("age"));
    }

    #[test]
    fn rejects_unknown_active_flag() {
        assert_eq!(parse_record("Alice;34;maybe"), Err("active"));
    }
}
