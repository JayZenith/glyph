pub fn parse_record(line: &str) -> Result<(&str, u32), &'static str> {
    let (key, value) = line.split_once(':').ok_or("missing colon")?;
    if value.is_empty() {
        return Err("missing value");
    }
    if !key.bytes().all(|b| b.is_ascii_lowercase() || b == b'_') {
        return Err("invalid key");
    }
    let num = value.parse::<u32>().map_err(|_| "invalid value")?;
    Ok((key, num))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("user_id:42"), Ok(("user_id", 42)));
    }

    #[test]
    fn rejects_missing_colon() {
        assert_eq!(parse_record("user_id"), Err("missing colon"));
    }

    #[test]
    fn rejects_empty_key() {
        assert_eq!(parse_record(":42"), Err("invalid key"));
    }

    #[test]
    fn rejects_invalid_key_chars() {
        assert_eq!(parse_record("User:42"), Err("invalid key"));
    }

    #[test]
    fn rejects_missing_value() {
        assert_eq!(parse_record("user_id:"), Err("missing value"));
    }
}
