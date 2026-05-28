#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub age: u8,
    pub score: u8,
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

        let name = parts[0].trim();
        if name.is_empty() {
            return Err(format!("line {}: empty name", idx + 1));
        }

        let age: u8 = parts[1]
            .trim()
            .parse()
            .map_err(|_| format!("line {}: invalid age", idx + 1))?;

        let score: u8 = parts[2]
            .trim()
            .parse()
            .map_err(|_| format!("line {}: invalid score", idx + 1))?;

        out.push(Record {
            name: name.to_string(),
            age,
            score,
        });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_lines_and_skips_blank_ones() {
        let input = "Alice,30,90\n\n Bob , 22 , 7 \n";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    name: "Alice".into(),
                    age: 30,
                    score: 90,
                },
                Record {
                    name: "Bob".into(),
                    age: 22,
                    score: 7,
                }
            ]
        );
    }

    #[test]
    fn rejects_extra_fields() {
        let err = parse_records("Alice,30,90,extra").unwrap_err();
        assert_eq!(err, "line 1: expected 3 fields");
    }

    #[test]
    fn rejects_score_above_100() {
        let err = parse_records("Alice,30,101").unwrap_err();
        assert_eq!(err, "line 1: score out of range");
    }

    #[test]
    fn rejects_empty_name() {
        let err = parse_records(" ,30,90").unwrap_err();
        assert_eq!(err, "line 1: empty name");
    }
}
