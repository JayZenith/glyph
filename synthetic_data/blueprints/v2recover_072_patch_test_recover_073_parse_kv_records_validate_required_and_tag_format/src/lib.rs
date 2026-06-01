#[derive(Debug, PartialEq, Eq)]
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
    let mut tags: Vec<String> = Vec::new();

    for part in input.split(';') {
        if part.is_empty() {
            continue;
        }
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("invalid field: {part}"))?;
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
                if !value.is_empty() {
                    tags = value.split(',').map(|s| s.to_string()).collect();
                }
            }
            _ => return Err(format!("unknown key: {key}")),
        }
    }

    Ok(Record {
        id: id.ok_or_else(|| "missing id".to_string())?,
        name: name.unwrap_or_default(),
        active: active.unwrap_or(false),
        tags,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let rec = parse_record("id=7;name=alpha;active=true;tags=red,blue").unwrap();
        assert_eq!(
            rec,
            Record {
                id: 7,
                name: "alpha".to_string(),
                active: true,
                tags: vec!["red".to_string(), "blue".to_string()],
            }
        );
    }

    #[test]
    fn requires_name_and_active_fields() {
        assert_eq!(parse_record("id=3;active=true").unwrap_err(), "missing name");
        assert_eq!(parse_record("id=3;name=zed").unwrap_err(), "missing active");
    }

    #[test]
    fn rejects_empty_or_non_lowercase_tags() {
        assert_eq!(
            parse_record("id=9;name=nova;active=false;tags=ok,,bad").unwrap_err(),
            "invalid tags"
        );
        assert_eq!(
            parse_record("id=9;name=nova;active=false;tags=ok,Bad").unwrap_err(),
            "invalid tags"
        );
    }
}
