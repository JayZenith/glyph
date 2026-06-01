pub fn parse_record(line: &str) -> Result<(&str, &str, u32), &'static str> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return Err("expected 3 fields");
    }

    let name = parts[0];
    let role = parts[1];
    let score = parts[2].parse::<u32>().map_err(|_| "invalid score")?;

    if name.is_empty() || role.is_empty() {
        return Err("empty field");
    }

    Ok((name, role, score))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("alice,admin,42"), Ok(("alice", "admin", 42)));
    }

    #[test]
    fn rejects_missing_field() {
        assert_eq!(parse_record("alice,admin"), Err("expected 3 fields"));
    }

    #[test]
    fn rejects_extra_field() {
        assert_eq!(parse_record("alice,admin,42,extra"), Err("expected 3 fields"));
    }

    #[test]
    fn rejects_invalid_score() {
        assert_eq!(parse_record("alice,admin,nope"), Err("invalid score"));
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(parse_record(",admin,42"), Err("empty field"));
    }
}
