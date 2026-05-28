#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub kind: String,
    pub id: u32,
    pub active: bool,
    pub note: Option<String>,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 4 {
        return Err("expected 4 fields".into());
    }

    let kind = parts[0].trim();
    if kind.is_empty() {
        return Err("missing kind".into());
    }

    let id: u32 = parts[1].trim().parse().map_err(|_| "invalid id")?;

    let active = match parts[2].trim() {
        "true" | "yes" | "1" => true,
        _ => false,
    };

    let note = {
        let raw = parts[3];
        if raw.is_empty() {
            None
        } else {
            Some(raw.to_string())
        }
    };

    Ok(Record {
        kind: kind.to_string(),
        id,
        active,
        note,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_record() {
        let rec = parse_record("user|42|true|hello").unwrap();
        assert_eq!(rec.kind, "user");
        assert_eq!(rec.id, 42);
        assert!(rec.active);
        assert_eq!(rec.note, Some("hello".to_string()));
    }

    #[test]
    fn rejects_unknown_boolean_token() {
        assert_eq!(parse_record("user|42|maybe|hello"), Err("invalid active".into()));
    }

    #[test]
    fn trims_empty_note_to_none() {
        let rec = parse_record("user|42|false|   ").unwrap();
        assert_eq!(rec.note, None);
    }

    #[test]
    fn alert_requires_note() {
        assert_eq!(parse_record("alert|7|yes|"), Err("note required for alert".into()));
    }

    #[test]
    fn accepts_false_tokens() {
        let rec = parse_record("job|9|no|done").unwrap();
        assert!(!rec.active);
        let rec2 = parse_record("job|9|0|done").unwrap();
        assert!(!rec2.active);
    }
}
