#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: String,
    pub age: u8,
}

pub fn parse_record(input: &str) -> Result<Record, String> {
    let mut id = None;
    let mut age = 0u8;

    for part in input.split(',') {
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("bad field: {}", part))?;
        match key {
            "id" => id = Some(value.to_string()),
            "age" => age = value.parse::<u8>().map_err(|_| "invalid age".to_string())?,
            _ => {}
        }
    }

    Ok(Record {
        id: id.unwrap_or_default(),
        age,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        assert_eq!(
            parse_record("id=u42,age=27").unwrap(),
            Record {
                id: "u42".to_string(),
                age: 27,
            }
        );
    }

    #[test]
    fn rejects_missing_required_fields() {
        assert!(parse_record("age=27").is_err());
        assert!(parse_record("id=u42").is_err());
    }

    #[test]
    fn rejects_unknown_keys() {
        assert!(parse_record("id=u42,age=27,name=bob").is_err());
    }

    #[test]
    fn rejects_age_out_of_range() {
        assert!(parse_record("id=u42,age=0").is_err());
        assert!(parse_record("id=u42,age=151").is_err());
    }
}
