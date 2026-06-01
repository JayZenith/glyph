#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub tags: Vec<String>,
}

pub fn parse_record(input: &str) -> Result<Record, String> {
    let mut id = None;
    let mut name = None;
    let mut active = None;
    let mut tags = None;

    for part in input.split(';') {
        if part.is_empty() {
            continue;
        }
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("bad field: {part}"))?;
        match key {
            "id" => {
                id = Some(value.parse::<u32>().map_err(|_| "invalid id".to_string())?);
            }
            "name" => {
                name = Some(value.to_string());
            }
            "active" => {
                active = Some(match value {
                    "true" => true,
                    "false" => false,
                    _ => return Err("invalid active".to_string()),
                });
            }
            "tags" => {
                let values = if value.is_empty() {
                    Vec::new()
                } else {
                    value.split(',').map(|s| s.to_string()).collect()
                };
                tags = Some(values);
            }
            _ => {}
        }
    }

    let record = Record {
        id: id.ok_or_else(|| "missing id".to_string())?,
        name: name.ok_or_else(|| "missing name".to_string())?,
        active: active.ok_or_else(|| "missing active".to_string())?,
        tags: tags.unwrap_or_default(),
    };

    if record.name.is_empty() {
        return Err("empty name".to_string());
    }

    Ok(record)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record_and_normalizes() {
        let got = parse_record("id=7;name= Alice ;active=TRUE;tags=red, blue ,green").unwrap();
        assert_eq!(
            got,
            Record {
                id: 7,
                name: "Alice".to_string(),
                active: true,
                tags: vec!["red".to_string(), "blue".to_string(), "green".to_string()],
            }
        );
    }

    #[test]
    fn rejects_unknown_and_duplicate_fields() {
        assert_eq!(
            parse_record("id=1;name=Bob;active=false;role=admin").unwrap_err(),
            "unknown field: role"
        );
        assert_eq!(
            parse_record("id=1;name=Bob;name=Rob;active=false").unwrap_err(),
            "duplicate field: name"
        );
    }

    #[test]
    fn validates_id_and_name_rules() {
        assert_eq!(parse_record("id=0;name=Bob;active=true").unwrap_err(), "invalid id");
        assert_eq!(parse_record("id=3;name=   ;active=true").unwrap_err(), "empty name");
    }

    #[test]
    fn validates_boolean_values_case_insensitively() {
        let a = parse_record("id=3;name=Bob;active=False").unwrap();
        assert!(!a.active);
        assert_eq!(
            parse_record("id=3;name=Bob;active=yes").unwrap_err(),
            "invalid active"
        );
    }

    #[test]
    fn validates_tags_are_nonempty_unique_ascii_alnum_or_dash() {
        assert_eq!(
            parse_record("id=9;name=Bob;active=true;tags=ok,,two").unwrap_err(),
            "invalid tag"
        );
        assert_eq!(
            parse_record("id=9;name=Bob;active=true;tags=dup,dup").unwrap_err(),
            "duplicate tag: dup"
        );
        assert_eq!(
            parse_record("id=9;name=Bob;active=true;tags=bad!,ok").unwrap_err(),
            "invalid tag"
        );
    }
}
