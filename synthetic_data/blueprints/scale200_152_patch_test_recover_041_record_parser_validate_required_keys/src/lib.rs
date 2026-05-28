#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: String,
    pub active: bool,
    pub score: Option<u32>,
}

pub fn parse_record(input: &str) -> Result<Record, String> {
    let mut id = String::new();
    let mut active = false;
    let mut score = None;

    for part in input.split(';') {
        if part.is_empty() {
            continue;
        }
        let Some((key, value)) = part.split_once('=') else {
            return Err("invalid field".into());
        };
        match key {
            "id" => id = value.to_string(),
            "active" => active = value == "true",
            "score" => {
                score = Some(value.parse::<u32>().map_err(|_| "invalid score")?);
            }
            _ => return Err("unknown field".into()),
        }
    }

    Ok(Record { id, active, score })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let rec = parse_record("id=abc;active=true;score=7").unwrap();
        assert_eq!(
            rec,
            Record {
                id: "abc".into(),
                active: true,
                score: Some(7)
            }
        );
    }

    #[test]
    fn rejects_invalid_active_value() {
        assert_eq!(parse_record("id=abc;active=yes").unwrap_err(), "invalid active");
    }

    #[test]
    fn rejects_missing_id() {
        assert_eq!(parse_record("active=false;score=2").unwrap_err(), "missing id");
    }

    #[test]
    fn rejects_empty_id() {
        assert_eq!(parse_record("id=;active=false").unwrap_err(), "missing id");
    }

    #[test]
    fn rejects_malformed_field_without_equals() {
        assert_eq!(parse_record("id=abc;active=true;score").unwrap_err(), "invalid field");
    }
}
