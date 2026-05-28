pub fn parse_record(line: &str) -> Result<(&str, &str), &'static str> {
    if line.is_empty() {
        return Err("empty");
    }

    let (key, value) = line.split_once('=').ok_or("missing separator")?;

    if key.is_empty() || value.is_empty() {
        return Err("empty field");
    }

    if !key.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') {
        return Err("invalid key");
    }

    Ok((key, value))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("user_id=42"), Ok(("user_id", "42")));
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(parse_record("user_id"), Err("missing separator"));
    }

    #[test]
    fn rejects_empty_fields() {
        assert_eq!(parse_record("=42"), Err("empty field"));
        assert_eq!(parse_record("user_id="), Err("empty field"));
    }

    #[test]
    fn rejects_invalid_key_chars() {
        assert_eq!(parse_record("user-id=42"), Err("invalid key"));
    }

    #[test]
    fn rejects_multiple_separators() {
        assert_eq!(parse_record("user=1=2"), Err("missing separator"));
    }
}
