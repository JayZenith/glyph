pub fn parse_records(input: &str) -> Result<Vec<(String, u32)>, String> {
    let mut out = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').map(str::trim).collect();
        if parts.len() < 2 {
            return Err(format!("line {}: expected name,age", idx + 1));
        }

        let name = parts[0];
        if name.is_empty() {
            return Err(format!("line {}: empty name", idx + 1));
        }

        let age: u32 = parts[1]
            .parse()
            .map_err(|_| format!("line {}: invalid age", idx + 1))?;

        out.push((name.to_string(), age));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_records;

    #[test]
    fn parses_valid_lines() {
        let input = "alice,30\nbob, 22\n";
        let got = parse_records(input).unwrap();
        assert_eq!(got, vec![("alice".to_string(), 30), ("bob".to_string(), 22)]);
    }

    #[test]
    fn rejects_missing_age() {
        let err = parse_records("alice\n").unwrap_err();
        assert_eq!(err, "line 1: expected name,age");
    }

    #[test]
    fn rejects_extra_fields() {
        let err = parse_records("alice,30,admin\n").unwrap_err();
        assert_eq!(err, "line 1: expected name,age");
    }

    #[test]
    fn rejects_empty_name() {
        let err = parse_records(",30\n").unwrap_err();
        assert_eq!(err, "line 1: empty name");
    }
}
