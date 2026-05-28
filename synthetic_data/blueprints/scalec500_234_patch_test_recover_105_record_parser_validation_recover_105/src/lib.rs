#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: String,
    pub qty: u32,
    pub active: bool,
    pub tags: Vec<String>,
}

pub fn parse_record(input: &str) -> Result<Record, String> {
    let mut id: Option<String> = None;
    let mut qty: Option<u32> = None;
    let mut active: Option<bool> = None;
    let mut tags: Vec<String> = Vec::new();

    for part in input.split(';') {
        if part.is_empty() {
            continue;
        }
        let (key, value) = part.split_once('=').ok_or_else(|| "missing =".to_string())?;
        match key {
            "id" => id = Some(value.to_string()),
            "qty" => qty = value.parse::<u32>().ok(),
            "active" => active = Some(value == "true"),
            "tags" => {
                if !value.is_empty() {
                    tags = value.split(',').map(|s| s.to_string()).collect();
                }
            }
            _ => {}
        }
    }

    Ok(Record {
        id: id.ok_or_else(|| "missing id".to_string())?,
        qty: qty.ok_or_else(|| "missing qty".to_string())?,
        active: active.ok_or_else(|| "missing active".to_string())?,
        tags,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let r = parse_record("id=AB-12;qty=7;active=false;tags=red,blue").unwrap();
        assert_eq!(r.id, "AB-12");
        assert_eq!(r.qty, 7);
        assert!(!r.active);
        assert_eq!(r.tags, vec!["red", "blue"]);
    }

    #[test]
    fn rejects_invalid_boolean_and_unknown_keys() {
        assert!(parse_record("id=AB-12;qty=1;active=yes").is_err());
        assert!(parse_record("id=AB-12;qty=1;active=true;extra=nope").is_err());
    }

    #[test]
    fn rejects_bad_ids_and_duplicate_or_empty_tags() {
        assert!(parse_record("id=ab-12;qty=1;active=true").is_err());
        assert!(parse_record("id=ABC-12;qty=1;active=true").is_err());
        assert!(parse_record("id=AB-12;qty=1;active=true;tags=red,,blue").is_err());
        assert!(parse_record("id=AB-12;qty=1;active=true;tags=red,red").is_err());
    }

    #[test]
    fn rejects_missing_fields_and_zero_qty() {
        assert!(parse_record("id=AB-12;active=true").is_err());
        assert!(parse_record("id=AB-12;qty=0;active=true").is_err());
    }
}
