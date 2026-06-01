pub fn parse_records(input: &str) -> Result<Vec<(String, u32)>, String> {
    let mut out = Vec::new();
    for (idx, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.split(',');
        let name = parts.next().ok_or_else(|| format!("line {}: missing name", idx + 1))?;
        let qty = parts.next().ok_or_else(|| format!("line {}: missing qty", idx + 1))?;

        if parts.next().is_some() {
            return Err(format!("line {}: too many fields", idx + 1));
        }

        if name.is_empty() {
            return Err(format!("line {}: empty name", idx + 1));
        }

        let qty: u32 = qty.parse().map_err(|_| format!("line {}: invalid qty", idx + 1))?;
        out.push((name.to_string(), qty));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_records;

    #[test]
    fn parses_comments_and_trims_fields() {
        let input = "# inventory\n apple , 10 \n\nbanana,2\n";
        let got = parse_records(input).unwrap();
        assert_eq!(got, vec![("apple".to_string(), 10), ("banana".to_string(), 2)]);
    }

    #[test]
    fn rejects_zero_quantity() {
        let err = parse_records("apple,0\n").unwrap_err();
        assert_eq!(err, "line 1: qty must be > 0");
    }

    #[test]
    fn rejects_extra_fields() {
        let err = parse_records("pear,3,ripe\n").unwrap_err();
        assert_eq!(err, "line 1: too many fields");
    }

    #[test]
    fn rejects_blank_name_after_trim() {
        let err = parse_records("   ,5\n").unwrap_err();
        assert_eq!(err, "line 1: empty name");
    }
}
