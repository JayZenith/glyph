pub fn parse_record(line: &str) -> Result<(&str, u32), &'static str> {
    let (name, count) = line.split_once(':').ok_or("missing colon")?;
    if name.is_empty() {
        return Err("empty name");
    }
    let count = count.parse::<u32>().map_err(|_| "invalid count")?;
    Ok((name, count))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("apple:12"), Ok(("apple", 12)));
    }

    #[test]
    fn rejects_missing_colon() {
        assert_eq!(parse_record("apple"), Err("missing colon"));
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(parse_record(":12"), Err("empty name"));
    }

    #[test]
    fn rejects_extra_colon() {
        assert_eq!(parse_record("apple:1:2"), Err("invalid count"));
    }

    #[test]
    fn rejects_whitespace_in_name() {
        assert_eq!(parse_record("red apple:3"), Err("invalid name"));
    }
}
