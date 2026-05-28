pub fn parse_record(line: &str) -> Result<(&str, u32, &str), &'static str> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return Err("expected 3 fields");
    }

    let id = parts[0];
    let qty = parts[1].parse::<u32>().map_err(|_| "invalid quantity")?;
    let name = parts[2];

    if id.is_empty() || name.is_empty() {
        return Err("empty field");
    }

    Ok((id, qty, name))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("A12,7,widget"), Ok(("A12", 7, "widget")));
    }

    #[test]
    fn rejects_missing_field() {
        assert_eq!(parse_record("A12,7"), Err("expected 3 fields"));
    }

    #[test]
    fn rejects_extra_field() {
        assert_eq!(parse_record("A12,7,widget,blue"), Err("expected 3 fields"));
    }

    #[test]
    fn rejects_invalid_quantity() {
        assert_eq!(parse_record("A12,nope,widget"), Err("invalid quantity"));
    }
}
