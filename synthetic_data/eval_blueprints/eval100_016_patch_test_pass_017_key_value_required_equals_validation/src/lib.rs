pub fn parse_record(line: &str) -> Result<(&str, &str), &'static str> {
    let line = line.trim();
    if line.is_empty() {
        return Err("empty line");
    }

    let Some((key, value)) = line.split_once('=') else {
        return Err("missing separator");
    };

    let key = key.trim();
    let value = value.trim();

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
        assert_eq!(parse_record("port = 8080"), Ok(("port", "8080")));
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(parse_record("port:8080"), Err("missing separator"));
    }

    #[test]
    fn rejects_empty_key() {
        assert_eq!(parse_record(" = value"), Err("empty key"));
    }

    #[test]
    fn rejects_empty_value() {
        assert_eq!(parse_record("port =   "), Err("empty value"));
    }

    #[test]
    fn rejects_blank_line() {
        assert_eq!(parse_record("   "), Err("empty line"));
    }
}
