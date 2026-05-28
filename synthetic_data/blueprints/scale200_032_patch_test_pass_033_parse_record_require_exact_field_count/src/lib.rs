pub fn parse_record(line: &str) -> Result<(&str, u16), &'static str> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() < 2 {
        return Err("invalid record");
    }

    let name = parts[0];
    if name.is_empty() {
        return Err("invalid record");
    }

    let score: u16 = parts[1].parse().map_err(|_| "invalid score")?;
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
    fn rejects_missing_name() {
        assert_eq!(parse_record(":42"), Err("invalid record"));
    }

    #[test]
    fn rejects_non_numeric_score() {
        assert_eq!(parse_record("alice:xx"), Err("invalid score"));
    }

    #[test]
    fn rejects_extra_fields() {
        assert_eq!(parse_record("alice:42:admin"), Err("invalid record"));
    }
}
