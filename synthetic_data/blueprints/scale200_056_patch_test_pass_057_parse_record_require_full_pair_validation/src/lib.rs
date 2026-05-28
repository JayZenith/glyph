#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub kind: String,
    pub id: u32,
    pub active: bool,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let mut kind = None;
    let mut id = None;
    let mut active = None;

    for part in line.split(';') {
        if part.is_empty() {
            continue;
        }
        let mut kv = part.splitn(2, '=');
        let key = kv.next().unwrap().trim();
        let value = kv.next().unwrap_or("").trim();

        match key {
            "kind" => {
                if value.is_empty() || !value.chars().all(|c| c.is_ascii_lowercase()) {
                    return Err("invalid kind".into());
                }
                kind = Some(value.to_string());
            }
            "id" => {
                let parsed = value.parse::<u32>().map_err(|_| "invalid id")?;
                id = Some(parsed);
            }
            "active" => match value {
                "true" => active = Some(true),
                "false" => active = Some(false),
                _ => return Err("invalid active".into()),
            },
            _ => return Err("unknown key".into()),
        }
    }

    Ok(Record {
        kind: kind.ok_or("missing kind")?,
        id: id.ok_or("missing id")?,
        active: active.unwrap_or(false),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let got = parse_record("kind=task;id=42;active=true").unwrap();
        assert_eq!(
            got,
            Record {
                kind: "task".into(),
                id: 42,
                active: true,
            }
        );
    }

    #[test]
    fn rejects_unknown_keys() {
        assert!(parse_record("kind=task;id=1;extra=nope").is_err());
    }

    #[test]
    fn rejects_missing_value_after_equals() {
        assert_eq!(parse_record("kind=;id=2;active=false"), Err("invalid kind".into()));
    }

    #[test]
    fn rejects_missing_equals_separator() {
        assert!(parse_record("kind=task;id=7;active").is_err());
    }

    #[test]
    fn defaults_active_only_when_absent() {
        let got = parse_record("kind=job;id=9").unwrap();
        assert_eq!(got.active, false);
    }
}
