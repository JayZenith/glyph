#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub kind: String,
    pub note: Option<String>,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let mut id = None;
    let mut kind = None;
    let mut note = None;

    for part in line.split(';') {
        let (key, value) = part.split_once('=').ok_or_else(|| "missing =".to_string())?;
        if value.is_empty() {
            return Err(format!("empty value for {key}"));
        }
        match key {
            "id" => {
                id = Some(value.parse::<u32>().map_err(|_| "bad id".to_string())?);
            }
            "kind" => {
                if value != "alpha" && value != "beta" {
                    return Err("bad kind".to_string());
                }
                kind = Some(value.to_string());
            }
            "note" => {
                note = Some(value.to_string());
            }
            _ => return Err("unknown field".to_string()),
        }
    }

    let id = id.ok_or_else(|| "missing id".to_string())?;
    let kind = kind.ok_or_else(|| "missing kind".to_string())?;

    Ok(Record { id, kind, note })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let rec = parse_record("id=42;kind=alpha;note=ready-7").unwrap();
        assert_eq!(
            rec,
            Record {
                id: 42,
                kind: "alpha".to_string(),
                note: Some("ready-7".to_string()),
            }
        );
    }

    #[test]
    fn rejects_missing_note() {
        assert!(parse_record("id=42;kind=alpha").is_err());
    }

    #[test]
    fn rejects_id_with_non_digits() {
        assert!(parse_record("id=7x;kind=beta;note=ok-1").is_err());
    }

    #[test]
    fn rejects_duplicate_fields() {
        assert!(parse_record("id=1;kind=alpha;note=x;note=y").is_err());
    }

    #[test]
    fn rejects_note_with_spaces() {
        assert!(parse_record("id=5;kind=beta;note=two words").is_err());
    }
}
