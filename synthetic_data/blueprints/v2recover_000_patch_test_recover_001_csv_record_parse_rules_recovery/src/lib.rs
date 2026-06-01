use std::collections::HashMap;

pub fn parse_records(input: &str) -> Result<HashMap<String, u32>, String> {
    let mut out = HashMap::new();
    for (idx, line) in input.lines().enumerate() {
        let line_no = idx + 1;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            return Err(format!("line {}: expected key,value", line_no));
        }

        let key = parts[0].trim();
        if key.is_empty() {
            return Err(format!("line {}: empty key", line_no));
        }

        let value: u32 = parts[1]
            .trim()
            .parse()
            .map_err(|_| format!("line {}: invalid number", line_no))?;

        out.insert(key.to_string(), value);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_records;

    #[test]
    fn parses_valid_rows_and_skips_blank_lines() {
        let got = parse_records("apple,10\n\npear, 7\n").unwrap();
        assert_eq!(got.len(), 2);
        assert_eq!(got.get("apple"), Some(&10));
        assert_eq!(got.get("pear"), Some(&7));
    }

    #[test]
    fn rejects_wrong_field_count() {
        let err = parse_records("apple,10,extra\n").unwrap_err();
        assert_eq!(err, "line 1: expected exactly 2 fields");
    }

    #[test]
    fn rejects_duplicate_keys() {
        let err = parse_records("apple,10\napple,12\n").unwrap_err();
        assert_eq!(err, "line 2: duplicate key 'apple'");
    }

    #[test]
    fn reports_invalid_number_on_second_field() {
        let err = parse_records("apple,nope\n").unwrap_err();
        assert_eq!(err, "line 1: invalid number 'nope'");
    }
}
