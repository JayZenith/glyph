#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub status: String,
    pub score: u32,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let mut id = None;
    let mut status = None;
    let mut score = None;

    for part in line.split(';') {
        let mut kv = part.splitn(2, '=');
        let key = kv.next().ok_or_else(|| "missing key".to_string())?;
        let value = kv.next().unwrap_or("");

        match key {
            "id" => {
                id = Some(value.parse::<u32>().map_err(|_| "invalid id".to_string())?);
            }
            "status" => {
                status = Some(value.to_string());
            }
            "score" => {
                let parsed = value.parse::<u32>().map_err(|_| "invalid score".to_string())?;
                if parsed > 100 {
                    return Err("score out of range".to_string());
                }
                score = Some(parsed);
            }
            _ => return Err("unknown field".to_string()),
        }
    }

    Ok(Record {
        id: id.ok_or_else(|| "missing id".to_string())?,
        status: status.ok_or_else(|| "missing status".to_string())?,
        score: score.ok_or_else(|| "missing score".to_string())?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let rec = parse_record("id=7;status=active;score=42").unwrap();
        assert_eq!(
            rec,
            Record {
                id: 7,
                status: "active".to_string(),
                score: 42,
            }
        );
    }

    #[test]
    fn rejects_unknown_status() {
        assert_eq!(parse_record("id=7;status=pending;score=42"), Err("invalid status".to_string()));
    }

    #[test]
    fn rejects_empty_status() {
        assert_eq!(parse_record("id=7;status=;score=42"), Err("empty value".to_string()));
    }

    #[test]
    fn rejects_missing_equals_in_field() {
        assert_eq!(parse_record("id=7;status=active;score"), Err("malformed field".to_string()));
    }

    #[test]
    fn rejects_out_of_range_score() {
        assert_eq!(parse_record("id=7;status=inactive;score=101"), Err("score out of range".to_string()));
    }
}
