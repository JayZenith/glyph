pub fn parse_record(line: &str) -> Result<(&str, u32, bool), &'static str> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return Err("expected 3 fields");
    }

    let name = parts[0];
    if name.is_empty() {
        return Err("empty name");
    }

    let count = parts[1].parse::<u32>().map_err(|_| "invalid count")?;
    let enabled = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("invalid enabled flag"),
    };

    Ok((name, count, enabled))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("widget,12,true"), Ok(("widget", 12, true)));
    }

    #[test]
    fn rejects_missing_field() {
        assert_eq!(parse_record("widget,12"), Err("expected 3 fields"));
    }

    #[test]
    fn rejects_extra_field() {
        assert_eq!(parse_record("widget,12,true,extra"), Err("expected 3 fields"));
    }

    #[test]
    fn rejects_bad_bool() {
        assert_eq!(parse_record("widget,12,yes"), Err("invalid enabled flag"));
    }
}
