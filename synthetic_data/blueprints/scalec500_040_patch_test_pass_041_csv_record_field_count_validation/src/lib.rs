pub fn parse_records(input: &str, expected_fields: usize) -> Result<Vec<Vec<String>>, String> {
    let mut out = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let fields: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
        if fields.is_empty() {
            return Err(format!("line {} has no fields", idx + 1));
        }
        out.push(fields);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_records;

    #[test]
    fn parses_valid_rows() {
        let input = "a, b, c\n1,2,3\n";
        let rows = parse_records(input, 3).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], vec!["a", "b", "c"]);
        assert_eq!(rows[1], vec!["1", "2", "3"]);
    }

    #[test]
    fn rejects_wrong_field_count() {
        let err = parse_records("x,y\n", 3).unwrap_err();
        assert_eq!(err, "line 1 expected 3 fields but got 2");
    }

    #[test]
    fn skips_blank_lines_but_still_reports_original_line_number() {
        let err = parse_records("\nname,age\nsolo\n", 2).unwrap_err();
        assert_eq!(err, "line 3 expected 2 fields but got 1");
    }
}
