#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub kind: String,
    pub active: bool,
    pub note: Option<String>,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let mut id = None;
    let mut kind = None;
    let mut active = None;
    let mut note = None;

    for field in line.split(';') {
        let field = field.trim();
        if field.is_empty() {
            continue;
        }

        let (key, value) = field
            .split_once('=')
            .ok_or_else(|| format!("missing '=' in field: {field}"))?;

        let key = key.trim();
        let value = value.trim();

        match key {
            "id" => {
                id = Some(value.parse::<u32>().map_err(|_| "invalid id".to_string())?);
            }
            "kind" => {
                kind = Some(value.to_string());
            }
            "active" => match value {
                "true" => active = Some(true),
                "false" => active = Some(false),
                _ => return Err("invalid active".to_string()),
            },
            "note" => {
                note = Some(value.to_string());
            }
            _ => return Err(format!("unknown key: {key}")),
        }
    }

    let id = id.ok_or_else(|| "missing id".to_string())?;
    let kind = kind.ok_or_else(|| "missing kind".to_string())?;
    let active = active.ok_or_else(|| "missing active".to_string())?;

    if kind != "alpha" && kind != "beta" && kind != "gamma" {
        return Err("invalid kind".to_string());
    }

    Ok(Record {
        id,
        kind,
        active,
        note,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record_with_note() {
        let rec = parse_record("id=7; kind=beta; active=true; note=ready").unwrap();
        assert_eq!(
            rec,
            Record {
                id: 7,
                kind: "beta".to_string(),
                active: true,
                note: Some("ready".to_string()),
            }
        );
    }

    #[test]
    fn parses_valid_record_without_note() {
        let rec = parse_record("id=9;kind=alpha;active=false").unwrap();
        assert_eq!(rec.note, None);
        assert_eq!(rec.kind, "alpha");
    }

    #[test]
    fn rejects_unknown_kind() {
        assert_eq!(parse_record("id=1;kind=delta;active=true"), Err("invalid kind".to_string()));
    }

    #[test]
    fn rejects_invalid_boolean() {
        assert_eq!(parse_record("id=1;kind=alpha;active=yes"), Err("invalid active".to_string()));
    }

    #[test]
    fn rejects_empty_required_value() {
        assert_eq!(parse_record("id=3;kind=;active=true"), Err("empty value for kind".to_string()));
    }

    #[test]
    fn allows_empty_optional_note() {
        let rec = parse_record("id=3;kind=gamma;active=true;note=").unwrap();
        assert_eq!(rec.note, Some(String::new()));
    }
}
