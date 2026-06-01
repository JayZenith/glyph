#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub kind: String,
    pub score: Option<i32>,
    pub active: bool,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();
    for (line_idx, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let mut id: Option<u32> = None;
        let mut kind: Option<String> = None;
        let mut score: Option<i32> = None;
        let mut active = false;

        for part in line.split(';') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            let Some((key, value)) = part.split_once('=') else {
                return Err(format!("line {}: invalid field", line_idx + 1));
            };
            match key.trim() {
                "id" => {
                    id = Some(value.trim().parse().map_err(|_| format!("line {}: invalid id", line_idx + 1))?);
                }
                "kind" => {
                    kind = Some(value.trim().to_string());
                }
                "score" => {
                    score = Some(value.trim().parse().map_err(|_| format!("line {}: invalid score", line_idx + 1))?);
                }
                "active" => {
                    active = matches!(value.trim(), "true" | "yes" | "1");
                }
                _ => {}
            }
        }

        let id = id.ok_or_else(|| format!("line {}: missing id", line_idx + 1))?;
        let kind = kind.ok_or_else(|| format!("line {}: missing kind", line_idx + 1))?;

        out.push(Record { id, kind, score, active });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records_with_defaults() {
        let input = "id=1;kind=alpha;score=7;active=yes\nid=2;kind=beta\nid=3;kind=gamma;active=false";
        let records = parse_records(input).unwrap();
        assert_eq!(
            records,
            vec![
                Record { id: 1, kind: "alpha".into(), score: Some(7), active: true },
                Record { id: 2, kind: "beta".into(), score: None, active: false },
                Record { id: 3, kind: "gamma".into(), score: None, active: false },
            ]
        );
    }

    #[test]
    fn rejects_unknown_keys() {
        let err = parse_records("id=1;kind=alpha;extra=nope").unwrap_err();
        assert_eq!(err, "line 1: unknown key extra");
    }

    #[test]
    fn rejects_duplicate_keys() {
        let err = parse_records("id=1;kind=alpha;id=2").unwrap_err();
        assert_eq!(err, "line 1: duplicate key id");
    }

    #[test]
    fn rejects_bad_boolean_values() {
        let err = parse_records("id=1;kind=alpha;active=maybe").unwrap_err();
        assert_eq!(err, "line 1: invalid active");
    }

    #[test]
    fn rejects_score_out_of_range() {
        let err = parse_records("id=1;kind=alpha;score=101").unwrap_err();
        assert_eq!(err, "line 1: score out of range");
    }

    #[test]
    fn rejects_empty_kind() {
        let err = parse_records("id=1;kind=").unwrap_err();
        assert_eq!(err, "line 1: empty kind");
    }
}
