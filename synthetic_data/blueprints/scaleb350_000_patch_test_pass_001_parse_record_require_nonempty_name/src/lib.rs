pub fn parse_record(line: &str) -> Option<(&str, u32)> {
    let (name, score_text) = line.split_once(':')?;
    let score = score_text.parse::<u32>().ok()?;
    Some((name, score))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("alice:42"), Some(("alice", 42)));
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(parse_record("alice"), None);
    }

    #[test]
    fn rejects_invalid_number() {
        assert_eq!(parse_record("alice:xx"), None);
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(parse_record(":42"), None);
    }
}
