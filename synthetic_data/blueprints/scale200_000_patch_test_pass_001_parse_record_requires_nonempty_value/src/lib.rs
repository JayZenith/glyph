pub fn parse_record(line: &str) -> Result<(&str, &str), &'static str> {
    let (key, value) = line.split_once('=') .ok_or("missing separator")?;

    if key.is_empty() {
        return Err("empty key");
    }

    Ok((key, value))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("mode=fast"), Ok(("mode", "fast")));
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(parse_record("mode"), Err("missing separator"));
    }

    #[test]
    fn rejects_empty_key() {
        assert_eq!(parse_record("=fast"), Err("empty key"));
    }

    #[test]
    fn rejects_empty_value() {
        assert_eq!(parse_record("mode="), Err("empty value"));
    }
}
