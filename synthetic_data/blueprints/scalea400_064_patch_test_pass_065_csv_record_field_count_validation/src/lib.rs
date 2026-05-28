pub fn parse_records(input: &str) -> Result<Vec<[String; 3]>, String> {
    let mut out = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split(',').map(|s| s.trim().to_string());
        let a = parts.next().ok_or_else(|| format!("line {}: missing id", idx + 1))?;
        let b = parts.next().ok_or_else(|| format!("line {}: missing name", idx + 1))?;
        let c = parts.next().ok_or_else(|| format!("line {}: missing score", idx + 1))?;

        out.push([a, b, c]);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_records;

    #[test]
    fn parses_valid_rows_and_skips_blank_lines() {
        let got = parse_records("1, Alice, 10\n\n2,Bob,20\n").unwrap();
        assert_eq!(got.len(), 2);
        assert_eq!(got[0], ["1".to_string(), "Alice".to_string(), "10".to_string()]);
        assert_eq!(got[1], ["2".to_string(), "Bob".to_string(), "20".to_string()]);
    }

    #[test]
    fn rejects_rows_with_extra_fields() {
        let err = parse_records("1,Alice,10,bonus").unwrap_err();
        assert_eq!(err, "line 1: expected 3 fields");
    }

    #[test]
    fn rejects_rows_with_missing_fields() {
        let err = parse_records("1,Alice").unwrap_err();
        assert_eq!(err, "line 1: missing score");
    }
}
