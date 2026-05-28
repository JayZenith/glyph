pub fn parse_record(line: &str) -> Result<(&str, u32), &'static str> {
    let mut parts = line.split(':');
    let name = parts.next().ok_or("missing name")?;
    let age_text = parts.next().ok_or("missing age")?;
    if parts.next().is_some() {
        return Err("too many fields");
    }
    if name.is_empty() {
        return Err("missing name");
    }
    let age: u32 = age_text.parse().map_err(|_| "invalid age")?;
    if age == 0 {
        return Err("invalid age");
    }
    Ok((name, age))
}

pub fn count_valid_records(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| parse_record(line).is_ok())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("alice:34"), Ok(("alice", 34)));
    }

    #[test]
    fn rejects_missing_name() {
        assert_eq!(parse_record(":34"), Err("missing name"));
    }

    #[test]
    fn rejects_non_numeric_age() {
        assert_eq!(parse_record("alice:xx"), Err("invalid age"));
    }

    #[test]
    fn rejects_zero_age() {
        assert_eq!(parse_record("alice:0"), Err("invalid age"));
    }

    #[test]
    fn rejects_extra_fields() {
        assert_eq!(parse_record("alice:34:admin"), Err("too many fields"));
    }

    #[test]
    fn counts_only_valid_non_empty_lines() {
        let input = "alice:34\n\n:20\nbob:19\ncarol:0\ndave:44:admin\n";
        assert_eq!(count_valid_records(input), 2);
    }
}
