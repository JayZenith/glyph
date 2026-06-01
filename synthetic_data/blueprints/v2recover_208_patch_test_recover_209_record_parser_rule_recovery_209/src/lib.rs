#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub key: String,
    pub qty: u32,
    pub active: bool,
    pub tags: Vec<String>,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();
    for (idx, raw_line) in input.lines().enumerate() {
        let line_no = idx + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 4 {
            return Err(format!("line {}: expected 4 fields", line_no));
        }

        let key = parts[0].trim().to_string();
        if key.is_empty() {
            return Err(format!("line {}: empty key", line_no));
        }

        let qty = parts[1].trim().parse::<u32>().unwrap_or(0);

        let active = matches!(parts[2].trim(), "true" | "yes" | "1");

        let tags = parts[3]
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        out.push(Record {
            key,
            qty,
            active,
            tags,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records_and_skips_comments() {
        let input = "# header
apple|10|true|fresh,local

berry-2|1|false|frozen
";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    key: "apple".into(),
                    qty: 10,
                    active: true,
                    tags: vec!["fresh".into(), "local".into()],
                },
                Record {
                    key: "berry-2".into(),
                    qty: 1,
                    active: false,
                    tags: vec!["frozen".into()],
                },
            ]
        );
    }

    #[test]
    fn rejects_invalid_key_characters() {
        let err = parse_records("bad key|3|true|ok\n").unwrap_err();
        assert_eq!(err, "line 1: invalid key");
    }

    #[test]
    fn rejects_invalid_quantity_text() {
        let err = parse_records("apple|xyz|true|ok\n").unwrap_err();
        assert_eq!(err, "line 1: invalid qty");
    }

    #[test]
    fn rejects_invalid_boolean_text() {
        let err = parse_records("apple|2|maybe|ok\n").unwrap_err();
        assert_eq!(err, "line 1: invalid active flag");
    }

    #[test]
    fn rejects_empty_and_duplicate_tags() {
        let err = parse_records("apple|2|true|red,,red\n").unwrap_err();
        assert_eq!(err, "line 1: invalid tags");
    }

    #[test]
    fn rejects_zero_quantity() {
        let err = parse_records("apple|0|true|ok\n").unwrap_err();
        assert_eq!(err, "line 1: qty must be positive");
    }
}
