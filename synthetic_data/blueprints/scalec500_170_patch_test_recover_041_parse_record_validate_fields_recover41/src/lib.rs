#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub active: bool,
    pub tags: Vec<String>,
}

pub fn parse_record(input: &str) -> Result<Record, String> {
    let mut id = None;
    let mut active = None;
    let mut tags = None;

    for part in input.split(';') {
        let mut kv = part.splitn(2, '=');
        let key = kv.next().ok_or_else(|| "missing key".to_string())?;
        let value = kv.next().ok_or_else(|| format!("missing value for {key}"))?;
        match key {
            "id" => {
                id = Some(value.parse::<u32>().map_err(|_| "invalid id".to_string())?);
            }
            "active" => {
                active = Some(match value {
                    "true" => true,
                    "false" => false,
                    _ => return Err("invalid active".to_string()),
                });
            }
            "tags" => {
                let parsed = if value.is_empty() {
                    Vec::new()
                } else {
                    value.split(',').map(|s| s.to_string()).collect()
                };
                tags = Some(parsed);
            }
            _ => {}
        }
    }

    let id = id.ok_or_else(|| "missing id".to_string())?;
    let active = active.ok_or_else(|| "missing active".to_string())?;
    let tags = tags.ok_or_else(|| "missing tags".to_string())?;

    if tags.iter().any(|t| t.is_empty()) {
        return Err("empty tag".to_string());
    }

    Ok(Record { id, active, tags })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let rec = parse_record("id=7;active=true;tags=red,blue").unwrap();
        assert_eq!(
            rec,
            Record {
                id: 7,
                active: true,
                tags: vec!["red".into(), "blue".into()]
            }
        );
    }

    #[test]
    fn trims_keys_values_and_tags() {
        let rec = parse_record(" id = 42 ; active = false ; tags = alpha, beta ,gamma ").unwrap();
        assert_eq!(rec.id, 42);
        assert!(!rec.active);
        assert_eq!(rec.tags, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn rejects_unknown_field() {
        let err = parse_record("id=1;active=true;tags=x;mode=fast").unwrap_err();
        assert_eq!(err, "unknown field: mode");
    }

    #[test]
    fn rejects_blank_tag_after_trimming() {
        let err = parse_record("id=3;active=false;tags=ok,   ,x").unwrap_err();
        assert_eq!(err, "empty tag");
    }

    #[test]
    fn requires_all_fields() {
        let err = parse_record("id=9;tags=a").unwrap_err();
        assert_eq!(err, "missing active");
    }
}
