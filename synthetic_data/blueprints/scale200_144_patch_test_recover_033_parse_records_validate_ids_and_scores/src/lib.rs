#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub id: u32,
    pub score: u8,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();

    for (line_no, raw) in input.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 3 {
            return Err(format!("line {}: expected 3 fields", line_no + 1));
        }

        let name = parts[0].trim();
        if name.is_empty() {
            return Err(format!("line {}: empty name", line_no + 1));
        }

        let id: u32 = parts[1]
            .trim()
            .parse()
            .map_err(|_| format!("line {}: invalid id", line_no + 1))?;

        let score: u8 = parts[2]
            .trim()
            .parse()
            .map_err(|_| format!("line {}: invalid score", line_no + 1))?;

        out.push(Record {
            name: name.to_string(),
            id,
            score,
        });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_input_and_skips_blank_lines() {
        let input = "alice|1|90\n\n bob | 2 | 75 \n";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    name: "alice".to_string(),
                    id: 1,
                    score: 90,
                },
                Record {
                    name: "bob".to_string(),
                    id: 2,
                    score: 75,
                },
            ]
        );
    }

    #[test]
    fn rejects_duplicate_ids() {
        let err = parse_records("alice|1|90\nbob|1|80\n").unwrap_err();
        assert_eq!(err, "line 2: duplicate id");
    }

    #[test]
    fn rejects_leading_zero_ids() {
        let err = parse_records("alice|01|90\n").unwrap_err();
        assert_eq!(err, "line 1: invalid id");
    }

    #[test]
    fn allows_zero_id_without_leading_zero_issue() {
        let got = parse_records("root|0|100\n").unwrap();
        assert_eq!(
            got,
            vec![Record {
                name: "root".to_string(),
                id: 0,
                score: 100,
            }]
        );
    }

    #[test]
    fn rejects_score_above_100() {
        let err = parse_records("alice|1|101\n").unwrap_err();
        assert_eq!(err, "line 1: invalid score");
    }
}
