pub fn parse_record(line: &str) -> Result<(&str, &str), &'static str> {
    if line.is_empty() {
        return Err("empty");
    }

    let (key, value) = line.split_once(':').ok_or("missing separator")?;

    if key.is_empty() || value.is_empty() {
        return Err("invalid field");
    }

    Ok((key, value))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_basic_record() {
        assert_eq!(parse_record("name:alice"), Ok(("name", "alice")));
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(parse_record("name"), Err("missing separator"));
    }

    #[test]
    fn rejects_empty_key_or_value() {
        assert_eq!(parse_record(":alice"), Err("invalid field"));
        assert_eq!(parse_record("name:"), Err("invalid field"));
    }

    #[test]
    fn rejects_multiple_separators() {
        assert_eq!(parse_record("name:alice:admin"), Err("missing separator"));
    }
}
