#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: String,
    pub score: u32,
    pub active: bool,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();

    for (line_no, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let mut id = String::new();
        let mut score = 0u32;
        let mut active = false;

        for part in line.split(';') {
            let (key, value) = part
                .split_once('=')
                .ok_or_else(|| format!("line {}: malformed field", line_no + 1))?;

            match key.trim() {
                "id" => id = value.trim().to_string(),
                "score" => {
                    score = value
                        .trim()
                        .parse::<u32>()
                        .map_err(|_| format!("line {}: invalid score", line_no + 1))?
                }
                "active" => {
                    active = match value.trim() {
                        "true" => true,
                        "false" => false,
                        _ => return Err(format!("line {}: invalid active", line_no + 1)),
                    }
                }
                _ => return Err(format!("line {}: unknown key", line_no + 1)),
            }
        }

        if id.is_empty() {
            return Err(format!("line {}: missing id", line_no + 1));
        }
        if !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(format!("line {}: invalid id", line_no + 1));
        }

        out.push(Record { id, score, active });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records() {
        let input = "id=abc-1;score=7;active=true\nid=z9;score=0;active=false";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    id: "abc-1".into(),
                    score: 7,
                    active: true,
                },
                Record {
                    id: "z9".into(),
                    score: 0,
                    active: false,
                }
            ]
        );
    }

    #[test]
    fn rejects_score_above_100() {
        let err = parse_records("id=a1;score=101;active=true").unwrap_err();
        assert_eq!(err, "line 1: invalid score");
    }

    #[test]
    fn rejects_missing_required_active_field() {
        let err = parse_records("id=a1;score=10").unwrap_err();
        assert_eq!(err, "line 1: missing active");
    }

    #[test]
    fn rejects_duplicate_keys() {
        let err = parse_records("id=a1;score=10;active=true;score=9").unwrap_err();
        assert_eq!(err, "line 1: duplicate key");
    }

    #[test]
    fn rejects_invalid_id_chars() {
        let err = parse_records("id=a_1;score=10;active=false").unwrap_err();
        assert_eq!(err, "line 1: invalid id");
    }
}
