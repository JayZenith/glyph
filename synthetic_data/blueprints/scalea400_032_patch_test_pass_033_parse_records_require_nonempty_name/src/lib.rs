pub fn parse_records(input: &str) -> Result<Vec<(u32, String)>, String> {
    let mut out = Vec::new();
    for (idx, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let mut parts = line.splitn(2, ':');
        let id_part = parts
            .next()
            .ok_or_else(|| format!("line {}: missing id", idx + 1))?;
        let name_part = parts
            .next()
            .ok_or_else(|| format!("line {}: missing name", idx + 1))?;

        let id = id_part
            .parse::<u32>()
            .map_err(|_| format!("line {}: invalid id", idx + 1))?;

        let name = name_part.to_string();
        out.push((id, name));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_records;

    #[test]
    fn parses_valid_lines() {
        let input = "1:alice\n2:bob";
        let records = parse_records(input).unwrap();
        assert_eq!(records, vec![(1, "alice".to_string()), (2, "bob".to_string())]);
    }

    #[test]
    fn skips_blank_lines() {
        let input = "1:alice\n\n2:bob\n";
        let records = parse_records(input).unwrap();
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn rejects_missing_separator() {
        let err = parse_records("1-alice").unwrap_err();
        assert_eq!(err, "line 1: missing name");
    }

    #[test]
    fn rejects_empty_name() {
        let err = parse_records("7:").unwrap_err();
        assert_eq!(err, "line 1: empty name");
    }
}
