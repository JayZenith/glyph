pub fn parse_records(input: &str) -> Result<Vec<Vec<(String, String)>>, String> {
    let mut out = Vec::new();

    for (line_no, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let mut fields = Vec::new();
        for part in line.split(',') {
            let piece = part.trim();
            let (key, value) = piece
                .split_once('=')
                .ok_or_else(|| format!("line {}: missing '='", line_no + 1))?;
            fields.push((key.trim().to_string(), value.trim().to_string()));
        }

        if fields.is_empty() {
            return Err(format!("line {}: empty record", line_no + 1));
        }

        out.push(fields);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_records;

    #[test]
    fn parses_trimmed_records() {
        let input = " id = 7 , name = Ada \nname=Bob,id=8";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                vec![
                    ("id".to_string(), "7".to_string()),
                    ("name".to_string(), "Ada".to_string())
                ],
                vec![
                    ("name".to_string(), "Bob".to_string()),
                    ("id".to_string(), "8".to_string())
                ]
            ]
        );
    }

    #[test]
    fn rejects_duplicate_keys_with_line_number() {
        let err = parse_records("id=1,name=A,id=2").unwrap_err();
        assert_eq!(err, "line 1: duplicate key 'id'");
    }

    #[test]
    fn rejects_empty_key_or_value() {
        let err1 = parse_records("=1").unwrap_err();
        assert_eq!(err1, "line 1: empty key");

        let err2 = parse_records("id=").unwrap_err();
        assert_eq!(err2, "line 1: empty value for key 'id'");
    }

    #[test]
    fn requires_id_field_in_each_record() {
        let err = parse_records("name=Ada").unwrap_err();
        assert_eq!(err, "line 1: missing required key 'id'");
    }
}
