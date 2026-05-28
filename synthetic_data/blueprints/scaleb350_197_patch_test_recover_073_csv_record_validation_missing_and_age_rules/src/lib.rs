#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: String,
    pub age: u8,
    pub active: bool,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 3 {
        return Err("expected 3 fields".into());
    }

    let id = parts[0].trim();
    let age_str = parts[1].trim();
    let active_str = parts[2].trim();

    if id.is_empty() {
        return Err("missing id".into());
    }

    let age: u8 = age_str.parse().map_err(|_| "invalid age")?;
    let active = match active_str {
        "true" => true,
        "false" => false,
        _ => return Err("invalid active".into()),
    };

    Ok(Record {
        id: id.to_string(),
        age,
        active,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record_with_trimmed_fields() {
        let rec = parse_record(" user-7 , 42 , true ").unwrap();
        assert_eq!(rec, Record {
            id: "user-7".to_string(),
            age: 42,
            active: true,
        });
    }

    #[test]
    fn rejects_missing_fields_after_trailing_separator() {
        assert_eq!(parse_record("abc,22,").unwrap_err(), "missing active");
    }

    #[test]
    fn rejects_age_out_of_range() {
        assert_eq!(parse_record("abc,130,true").unwrap_err(), "age out of range");
    }

    #[test]
    fn rejects_non_lowercase_boolean() {
        assert_eq!(parse_record("abc,22,TRUE").unwrap_err(), "invalid active");
    }
}
