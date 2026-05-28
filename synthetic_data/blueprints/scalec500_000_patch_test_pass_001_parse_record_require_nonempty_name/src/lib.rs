pub fn parse_record(line: &str) -> Result<(&str, u32), &'static str> {
    let mut parts = line.split(':');
    let name = parts.next().ok_or("missing name")?;
    let value = parts.next().ok_or("missing value")?;

    if parts.next().is_some() {
        return Err("too many fields");
    }

    let score = value.parse::<u32>().map_err(|_| "invalid value")?;
    Ok((name, score))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("alice:42"), Ok(("alice", 42)));
    }

    #[test]
    fn rejects_non_numeric_value() {
        assert_eq!(parse_record("alice:xx"), Err("invalid value"));
    }

    #[test]
    fn rejects_missing_name() {
        assert_eq!(parse_record(":42"), Err("missing name"));
    }

    #[test]
    fn rejects_extra_fields() {
        assert_eq!(parse_record("alice:42:admin"), Err("too many fields"));
    }
}
