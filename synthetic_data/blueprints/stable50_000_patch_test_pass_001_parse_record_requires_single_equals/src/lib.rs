pub fn parse_record(input: &str) -> Result<(&str, &str), &'static str> {
    let mut parts = input.split('=');
    let key = parts.next().ok_or("missing key")?;
    let value = parts.next().ok_or("missing value")?;

    if key.is_empty() || value.is_empty() {
        return Err("empty field");
    }

    Ok((key, value))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_simple_pair() {
        assert_eq!(parse_record("mode=fast"), Ok(("mode", "fast")));
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(parse_record("mode"), Err("missing value"));
    }

    #[test]
    fn rejects_empty_fields() {
        assert_eq!(parse_record("=fast"), Err("empty field"));
        assert_eq!(parse_record("mode="), Err("empty field"));
    }

    #[test]
    fn rejects_extra_separator() {
        assert_eq!(parse_record("mode=fast=now"), Err("invalid format"));
    }
}
