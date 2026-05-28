#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub active: bool,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let mut id = None;
    let mut name = None;
    let mut active = None;

    for part in line.split(';') {
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("invalid segment: {part}"))?;

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
            _ => {}
        }
    }

    match (id, name, active) {
        (Some(id), Some(name), Some(active)) => Ok(Record { id, name, active }),
        _ => Err("missing field".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let rec = parse_record("id=7;name=alice;active=true").unwrap();
        assert_eq!(rec, Record {
            id: 7,
            name: "alice".to_string(),
            active: true,
        });
    }

    #[test]
    fn rejects_unknown_field() {
        assert!(parse_record("id=7;name=alice;active=true;role=admin").is_err());
    }

    #[test]
    fn rejects_empty_name() {
        assert!(parse_record("id=7;name=;active=true").is_err());
    }

    #[test]
    fn rejects_missing_equals() {
        assert!(parse_record("id=7;name=alice;active").is_err());
    }

    #[test]
    fn rejects_invalid_bool() {
        assert!(parse_record("id=7;name=alice;active=yes").is_err());
    }
}
