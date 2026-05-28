#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub age: u8,
    pub score: u16,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();

    for (idx, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 3 {
            return Err(format!("line {}: expected 3 fields", idx + 1));
        }

        let name = parts[0].trim().to_string();
        let age: u8 = parts[1]
            .trim()
            .parse()
            .map_err(|_| format!("line {}: invalid age", idx + 1))?;
        let score: u16 = parts[2]
            .trim()
            .parse()
            .map_err(|_| format!("line {}: invalid score", idx + 1))?;

        if age == 0 {
            return Err(format!("line {}: invalid age", idx + 1));
        }

        out.push(Record { name, age, score });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_rows_with_spaces() {
        let input = " Alice , 30 , 88\nBob,19,100\n";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    name: "Alice".into(),
                    age: 30,
                    score: 88,
                },
                Record {
                    name: "Bob".into(),
                    age: 19,
                    score: 100,
                }
            ]
        );
    }

    #[test]
    fn rejects_wrong_field_count() {
        let err = parse_records("A,20\n").unwrap_err();
        assert!(err.contains("expected 3 fields"));

        let err = parse_records("A,20,90,extra\n").unwrap_err();
        assert!(err.contains("expected 3 fields"));
    }

    #[test]
    fn rejects_blank_name_and_out_of_range_score() {
        let err = parse_records(" ,20,90\n").unwrap_err();
        assert!(err.contains("empty name"));

        let err = parse_records("Cara,20,101\n").unwrap_err();
        assert!(err.contains("invalid score"));
    }

    #[test]
    fn rejects_zero_age() {
        let err = parse_records("Dana,0,50\n").unwrap_err();
        assert!(err.contains("invalid age"));
    }
}
