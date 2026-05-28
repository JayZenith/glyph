pub fn parse_entry(line: &str) -> Result<(&str, &str), &'static str> {
    let (key, value) = line.split_once('=').ok_or("missing separator")?;

    if key.is_empty() {
        return Err("empty key");
    }
    if value.is_empty() {
        return Err("empty value");
    }
    if key.bytes().any(|b| !(b.is_ascii_alphanumeric() || b == b'_')) {
        return Err("invalid key");
    }

    Ok((key, value))
}

#[cfg(test)]
mod tests {
    use super::parse_entry;

    #[test]
    fn parses_valid_entry() {
        assert_eq!(parse_entry("mode=fast"), Ok(("mode", "fast")));
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(parse_entry("mode"), Err("missing separator"));
    }

    #[test]
    fn rejects_multiple_separators() {
        assert_eq!(parse_entry("mode=fast=extra"), Err("missing separator"));
    }

    #[test]
    fn rejects_invalid_key() {
        assert_eq!(parse_entry("bad-key=value"), Err("invalid key"));
    }

    #[test]
    fn rejects_empty_value() {
        assert_eq!(parse_entry("mode="), Err("empty value"));
    }
}
