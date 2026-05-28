#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub age: u8,
    pub role: String,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return Err("expected 3 fields".into());
    }

    let name = parts[0].trim();
    let age = parts[1].trim().parse::<u8>().unwrap_or(0);
    let role = parts[2].trim();

    if name.is_empty() || role.is_empty() {
        return Err("empty field".into());
    }

    Ok(Record {
        name: name.to_string(),
        age,
        role: role.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let rec = parse_record("Ada,36,admin").unwrap();
        assert_eq!(rec, Record {
            name: "Ada".to_string(),
            age: 36,
            role: "admin".to_string(),
        });
    }

    #[test]
    fn trims_fields() {
        let rec = parse_record("  Bob  , 7 , user ").unwrap();
        assert_eq!(rec.name, "Bob");
        assert_eq!(rec.age, 7);
        assert_eq!(rec.role, "user");
    }

    #[test]
    fn rejects_missing_field() {
        assert!(parse_record("Ada,36").is_err());
    }

    #[test]
    fn rejects_extra_field() {
        assert!(parse_record("Ada,36,admin,extra").is_err());
    }

    #[test]
    fn rejects_invalid_age_text() {
        assert!(parse_record("Ada,xx,admin").is_err());
    }

    #[test]
    fn rejects_age_above_limit() {
        assert!(parse_record("Ada,131,admin").is_err());
    }

    #[test]
    fn rejects_empty_name() {
        assert!(parse_record(" ,12,user").is_err());
    }

    #[test]
    fn rejects_empty_role() {
        assert!(parse_record("Ada,12,  ").is_err());
    }
}
