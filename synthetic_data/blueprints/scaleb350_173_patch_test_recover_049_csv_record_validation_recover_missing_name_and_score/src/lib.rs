pub fn parse_records(input: &str) -> Result<Vec<(u32, String, u8)>, String> {
    let mut out = Vec::new();
    for (line_idx, raw) in input.lines().enumerate() {
        let line_no = line_idx + 1;
        if raw.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = raw.split(',').collect();
        if parts.len() != 3 {
            return Err(format!("line {}: expected 3 fields", line_no));
        }

        let id: u32 = parts[0]
            .parse()
            .map_err(|_| format!("line {}: invalid id", line_no))?;
        let name = parts[1].to_string();
        let score: u8 = parts[2]
            .parse()
            .map_err(|_| format!("line {}: invalid score", line_no))?;

        if score > 100 {
            return Err(format!("line {}: score out of range", line_no));
        }

        out.push((id, name, score));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_records;

    #[test]
    fn parses_valid_rows_and_skips_blank_lines() {
        let input = "1,Alice,90\n\n2,Bob,0\n";
        let rows = parse_records(input).unwrap();
        assert_eq!(rows, vec![(1, "Alice".to_string(), 90), (2, "Bob".to_string(), 0)]);
    }

    #[test]
    fn rejects_missing_name_even_if_spaces_only() {
        let err = parse_records("7,   ,10\n").unwrap_err();
        assert_eq!(err, "line 1: missing name");
    }

    #[test]
    fn rejects_score_above_100() {
        let err = parse_records("3,Cara,101\n").unwrap_err();
        assert_eq!(err, "line 1: score out of range");
    }

    #[test]
    fn trims_fields_before_parsing_numbers_and_name() {
        let rows = parse_records(" 42 , Dana , 8 \n").unwrap();
        assert_eq!(rows, vec![(42, "Dana".to_string(), 8)]);
    }
}
