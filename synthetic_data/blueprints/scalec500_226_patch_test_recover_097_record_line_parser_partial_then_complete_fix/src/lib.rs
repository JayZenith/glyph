#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub score: Option<u8>,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();

    for (line_no, raw) in input.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        let mut id = None;
        let mut name = None;
        let mut score = None;

        for part in line.split(';') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            let (key, value) = match part.split_once('=') {
                Some((k, v)) => (k.trim(), v.trim()),
                None => return Err(format!("line {}: bad field", line_no + 1)),
            };

            match key {
                "id" => {
                    id = value.parse::<u32>().ok();
                }
                "name" => {
                    name = Some(value.to_string());
                }
                "score" => {
                    score = value.parse::<u8>().ok();
                }
                _ => {}
            }
        }

        let id = id.ok_or_else(|| format!("line {}: missing id", line_no + 1))?;
        let name = name.ok_or_else(|| format!("line {}: missing name", line_no + 1))?;
        out.push(Record { id, name, score });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records() {
        let input = "id=1;name=Alice;score=9\nid=2;name=Bob";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    id: 1,
                    name: "Alice".into(),
                    score: Some(9)
                },
                Record {
                    id: 2,
                    name: "Bob".into(),
                    score: None
                }
            ]
        );
    }

    #[test]
    fn rejects_invalid_score_and_empty_name() {
        assert!(parse_records("id=1;name=Ann;score=300").is_err());
        assert!(parse_records("id=1;name=").is_err());
    }

    #[test]
    fn rejects_unknown_or_duplicate_fields() {
        assert!(parse_records("id=1;name=A;extra=x").is_err());
        assert!(parse_records("id=1;name=A;id=2").is_err());
    }
}
