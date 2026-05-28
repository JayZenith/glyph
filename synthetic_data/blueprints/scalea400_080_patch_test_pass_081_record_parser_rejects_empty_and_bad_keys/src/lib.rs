#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub active: bool,
}

pub fn parse_record(input: &str) -> Result<Record, String> {
    let mut id = None;
    let mut name = None;
    let mut active = false;

    for part in input.split(',') {
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("invalid segment: {part}"))?;

        match key {
            "id" => id = value.parse::<u32>().ok(),
            "name" => name = Some(value.to_string()),
            "active" => match value {
                "true" => active = true,
                "false" => active = false,
                _ => return Err("invalid active".into()),
            },
            _ => {}
        }
    }

    Ok(Record {
        id: id.ok_or_else(|| "missing id".to_string())?,
        name: name.ok_or_else(|| "missing name".to_string())?,
        active,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_record() {
        let rec = parse_record("id=42,name=Ada,active=true").unwrap();
        assert_eq!(
            rec,
            Record {
                id: 42,
                name: "Ada".into(),
                active: true,
            }
        );
    }

    #[test]
    fn defaults_active_to_false() {
        let rec = parse_record("id=7,name=Bob").unwrap();
        assert!(!rec.active);
    }

    #[test]
    fn rejects_unknown_key() {
        assert!(parse_record("id=1,name=Zoe,role=admin").is_err());
    }

    #[test]
    fn rejects_duplicate_id() {
        assert!(parse_record("id=1,id=2,name=Zoe").is_err());
    }

    #[test]
    fn rejects_empty_name() {
        assert!(parse_record("id=9,name=").is_err());
    }

    #[test]
    fn rejects_non_digit_id() {
        assert!(parse_record("id=12x,name=Kai").is_err());
    }

    #[test]
    fn rejects_empty_segment() {
        assert!(parse_record("id=3,,name=Eve").is_err());
    }
}
