#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub age: u8,
    pub active: bool,
}

pub fn parse_record(input: &str) -> Result<Record, String> {
    let mut name = None;
    let mut age = None;
    let mut active = None;

    for part in input.split(';') {
        let mut kv = part.splitn(2, '=');
        let key = kv.next().ok_or("missing key")?;
        let value = kv.next().ok_or("missing value")?;

        match key {
            "name" => name = Some(value.to_string()),
            "age" => age = Some(value.parse::<u8>().map_err(|_| "bad age")?),
            "active" => active = Some(value == "true"),
            _ => {}
        }
    }

    let name = name.ok_or("missing name")?;
    let age = age.ok_or("missing age")?;
    let active = active.ok_or("missing active")?;

    if age > 120 {
        return Err("age out of range".to_string());
    }

    Ok(Record { name, age, active })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let rec = parse_record("name=Ana;age=42;active=true").unwrap();
        assert_eq!(rec.name, "Ana");
        assert_eq!(rec.age, 42);
        assert!(rec.active);
    }

    #[test]
    fn rejects_unknown_key() {
        assert!(parse_record("name=Ana;age=42;active=true;role=admin").is_err());
    }

    #[test]
    fn rejects_invalid_bool() {
        assert!(parse_record("name=Ana;age=42;active=yes").is_err());
    }

    #[test]
    fn rejects_blank_name_and_zero_age() {
        assert!(parse_record("name=;age=0;active=false").is_err());
    }

    #[test]
    fn rejects_malformed_field_without_equals() {
        assert!(parse_record("name=Ana;age=42;active=true;broken").is_err());
    }
}
