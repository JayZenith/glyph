pub fn parse_records(input: &str) -> Result<Vec<(String, u32)>, String> {
    let mut out = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            return Err(format!("line {}: expected 2 fields", idx + 1));
        }

        let name = parts[0].trim();
        if name.is_empty() {
            return Err(format!("line {}: empty name", idx + 1));
        }

        let qty = parts[1]
            .trim()
            .parse::<u32>()
            .map_err(|_| format!("line {}: invalid quantity", idx + 1))?;

        out.push((name.to_string(), qty));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_records;

    #[test]
    fn parses_valid_lines() {
        let input = "apples,10\npears, 4\n";
        let got = parse_records(input).unwrap();
        assert_eq!(got, vec![("apples".to_string(), 10), ("pears".to_string(), 4)]);
    }

    #[test]
    fn rejects_missing_field() {
        let err = parse_records("apples\n").unwrap_err();
        assert_eq!(err, "line 1: expected 2 fields");
    }

    #[test]
    fn rejects_extra_field() {
        let err = parse_records("apples,10,fresh\n").unwrap_err();
        assert_eq!(err, "line 1: expected 2 fields");
    }

    #[test]
    fn rejects_empty_name() {
        let err = parse_records(",10\n").unwrap_err();
        assert_eq!(err, "line 1: empty name");
    }
}
