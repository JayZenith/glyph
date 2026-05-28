pub fn parse_record(line: &str) -> Result<(&str, u32, bool), &'static str> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return Err("expected 3 fields");
    }

    let name = parts[0];
    if name.is_empty() {
        return Err("empty name");
    }

    let qty = parts[1].parse::<u32>().map_err(|_| "bad quantity")?;
    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("bad active flag"),
    };

    Ok((name, qty, active))
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
    fn rejects_bad_quantity() {
        assert_eq!(parse_record("widget,nope,true"), Err("bad quantity"));
    }
}
