#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub score: u8,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();

    for (line_idx, raw) in input.lines().enumerate() {
        let line_no = line_idx + 1;
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut id = None;
        let mut name = None;
        let mut active = None;
        let mut score = None;

        for part in line.split(';') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            let (key, value) = part
                .split_once('=')
                .ok_or_else(|| format!("line {}: invalid field", line_no))?;
            match key.trim() {
                "id" => {
                    id = Some(value.trim().parse::<u32>().map_err(|_| format!("line {}: bad id", line_no))?);
                }
                "name" => {
                    name = Some(value.trim().to_string());
                }
                "active" => {
                    active = Some(value.trim() == "true");
                }
                "score" => {
                    score = Some(value.trim().parse::<u8>().map_err(|_| format!("line {}: bad score", line_no))?);
                }
                _ => {}
            }
        }

        let record = Record {
            id: id.ok_or_else(|| format!("line {}: missing id", line_no))?,
            name: name.ok_or_else(|| format!("line {}: missing name", line_no))?,
            active: active.ok_or_else(|| format!("line {}: missing active", line_no))?,
            score: score.ok_or_else(|| format!("line {}: missing score", line_no))?,
        };

        out.push(record);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records_with_comments_and_spacing() {
        let input = "
            # header
            id=1; name=Alice ; active=true; score=42

            id=2;name=Bob;active=false;score=0
        ";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record { id: 1, name: "Alice".into(), active: true, score: 42 },
                Record { id: 2, name: "Bob".into(), active: false, score: 0 },
            ]
        );
    }

    #[test]
    fn rejects_unknown_field() {
        let err = parse_records("id=1;name=A;active=true;score=4;extra=nope").unwrap_err();
        assert_eq!(err, "line 1: unknown field extra");
    }

    #[test]
    fn rejects_duplicate_field() {
        let err = parse_records("id=1;name=A;active=true;score=4;score=5").unwrap_err();
        assert_eq!(err, "line 1: duplicate field score");
    }

    #[test]
    fn rejects_invalid_boolean_literal() {
        let err = parse_records("id=1;name=A;active=yes;score=4").unwrap_err();
        assert_eq!(err, "line 1: bad active");
    }

    #[test]
    fn rejects_score_above_100() {
        let err = parse_records("id=1;name=A;active=true;score=101").unwrap_err();
        assert_eq!(err, "line 1: score out of range");
    }

    #[test]
    fn rejects_blank_name() {
        let err = parse_records("id=1;name=   ;active=true;score=4").unwrap_err();
        assert_eq!(err, "line 1: blank name");
    }
}
