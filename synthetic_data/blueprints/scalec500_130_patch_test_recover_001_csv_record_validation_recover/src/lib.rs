pub fn parse_records(input: &str) -> Result<Vec<(String, u32)>, String> {
    let mut out = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split(',');
        let name = parts.next().unwrap_or("");
        let qty_text = parts.next().unwrap_or("");

        if parts.next().is_some() {
            return Err(format!("line {}: expected 2 fields", idx + 1));
        }

        let qty = qty_text
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
    fn parses_valid_rows_and_skips_blank_lines() {
        let input = "apple,10\n\npear,2\n";
        let got = parse_records(input).unwrap();
        assert_eq!(got, vec![("apple".to_string(), 10), ("pear".to_string(), 2)]);
    }

    #[test]
    fn trims_fields_before_validation() {
        let input = "  kiwi  , 7 ";
        let got = parse_records(input).unwrap();
        assert_eq!(got, vec![("kiwi".to_string(), 7)]);
    }

    #[test]
    fn rejects_empty_name_after_trimming() {
        let err = parse_records("   ,4").unwrap_err();
        assert_eq!(err, "line 1: empty name");
    }

    #[test]
    fn rejects_zero_quantity() {
        let err = parse_records("plum,0").unwrap_err();
        assert_eq!(err, "line 1: quantity must be > 0");
    }

    #[test]
    fn rejects_extra_fields() {
        let err = parse_records("melon,3,extra").unwrap_err();
        assert_eq!(err, "line 1: expected 2 fields");
    }
}
