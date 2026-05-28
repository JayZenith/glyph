use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: String,
    pub kind: String,
    pub active: bool,
}

pub fn parse_record(input: &str) -> Result<Record, String> {
    let mut id = None;
    let mut kind = None;
    let mut active = None;
    let mut seen = HashSet::new();

    for part in input.split(',') {
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("invalid field: {}", part))?;

        if !seen.insert(key) {
            return Err(format!("duplicate key: {}", key));
        }

        match key {
            "id" => id = Some(value.to_string()),
            "kind" => kind = Some(value.to_string()),
            "active" => match value {
                "true" => active = Some(true),
                "false" => active = Some(false),
                _ => return Err(format!("invalid active: {}", value)),
            },
            _ => {}
        }
    }

    Ok(Record {
        id: id.ok_or_else(|| "missing id".to_string())?,
        kind: kind.ok_or_else(|| "missing kind".to_string())?,
        active: active.ok_or_else(|| "missing active".to_string())?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let rec = parse_record("id=42,kind=widget,active=true").unwrap();
        assert_eq!(
            rec,
            Record {
                id: "42".into(),
                kind: "widget".into(),
                active: true,
            }
        );
    }

    #[test]
    fn rejects_unknown_key() {
        let err = parse_record("id=42,kind=widget,color=blue,active=true").unwrap_err();
        assert_eq!(err, "unknown key: color");
    }

    #[test]
    fn rejects_empty_value() {
        let err = parse_record("id=,kind=widget,active=true").unwrap_err();
        assert_eq!(err, "empty value for id");
    }

    #[test]
    fn rejects_duplicate_key() {
        let err = parse_record("id=42,kind=widget,id=7,active=true").unwrap_err();
        assert_eq!(err, "duplicate key: id");
    }

    #[test]
    fn rejects_missing_required_field() {
        let err = parse_record("id=42,active=false").unwrap_err();
        assert_eq!(err, "missing kind");
    }

    #[test]
    fn rejects_invalid_active_value() {
        let err = parse_record("id=42,kind=widget,active=yes").unwrap_err();
        assert_eq!(err, "invalid active: yes");
    }

    #[test]
    fn rejects_field_without_equals() {
        let err = parse_record("id=42,kind,active=true").unwrap_err();
        assert_eq!(err, "invalid field: kind");
    }
}
