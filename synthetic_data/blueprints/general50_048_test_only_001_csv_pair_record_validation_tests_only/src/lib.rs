pub fn parse_pairs(input: &str) -> Result<Vec<(String, u32)>, String> {
    let mut out = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split(',');
        let name = parts
            .next()
            .ok_or_else(|| format!("line {}: missing name", idx + 1))?
            .trim();
        let value = parts
            .next()
            .ok_or_else(|| format!("line {}: missing value", idx + 1))?
            .trim();

        if parts.next().is_some() {
            return Err(format!("line {}: expected exactly 2 fields", idx + 1));
        }
        if name.is_empty() {
            return Err(format!("line {}: empty name", idx + 1));
        }
        if !name.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(format!("line {}: invalid name", idx + 1));
        }

        let parsed = value
            .parse::<u32>()
            .map_err(|_| format!("line {}: invalid value", idx + 1))?;
        out.push((name.to_string(), parsed));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_pairs;

    #[test]
    fn parses_valid_records_and_skips_blank_lines() {
        let input = "alice,10\n\nBob,7\ncharlie,0\n";
        let got = parse_pairs(input).unwrap();
        assert_eq!(
            got,
            vec![
                ("alice".to_string(), 10),
                ("Bob".to_string(), 7),
                ("charlie".to_string(), 0)
            ]
        );
    }

    #[test]
    fn rejects_extra_fields() {
        let err = parse_pairs("alice,10,extra\n").unwrap_err();
        assert_eq!(err, "line 1: expected exactly 2 fields");
    }

    #[test]
    fn rejects_empty_name() {
        let err = parse_pairs(",12\n").unwrap_err();
        assert_eq!(err, "line 1: empty name");
    }

    #[test]
    fn rejects_non_alpha_name() {
        let err = parse_pairs("al1ce,12\n").unwrap_err();
        assert_eq!(err, "line 1: invalid name");
    }

    #[test]
    fn rejects_invalid_number() {
        let err = parse_pairs("alice,ten\n").unwrap_err();
        assert_eq!(err, "line 1: invalid value");
    }
}
