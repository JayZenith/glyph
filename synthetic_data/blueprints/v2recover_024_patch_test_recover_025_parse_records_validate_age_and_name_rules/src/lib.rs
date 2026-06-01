#[derive(Debug, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

pub fn parse_person(input: &str) -> Result<Person, String> {
    let mut name = None;
    let mut age = None;

    for part in input.split(';') {
        if part.is_empty() {
            continue;
        }
        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| "invalid field".to_string())?;
        match key.trim() {
            "name" => name = Some(value.to_string()),
            "age" => {
                let parsed = value.parse::<u8>().map_err(|_| "invalid age".to_string())?;
                age = Some(parsed);
            }
            _ => return Err("unknown field".to_string()),
        }
    }

    let name = name.ok_or_else(|| "missing name".to_string())?;
    let age = age.ok_or_else(|| "missing age".to_string())?;

    Ok(Person { name, age })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record_with_trimmed_fields() {
        assert_eq!(
            parse_person(" name = Ada Lovelace ; age = 36 ").unwrap(),
            Person {
                name: "Ada Lovelace".to_string(),
                age: 36,
            }
        );
    }

    #[test]
    fn rejects_missing_required_fields() {
        assert_eq!(parse_person("name=Ada").unwrap_err(), "missing age");
        assert_eq!(parse_person("age=20").unwrap_err(), "missing name");
    }

    #[test]
    fn rejects_invalid_age_values() {
        assert_eq!(parse_person("name=Ada;age=abc").unwrap_err(), "invalid age");
        assert_eq!(parse_person("name=Ada;age=0").unwrap_err(), "invalid age");
        assert_eq!(parse_person("name=Ada;age=151").unwrap_err(), "invalid age");
    }

    #[test]
    fn rejects_invalid_names() {
        assert_eq!(parse_person("name=   ;age=20").unwrap_err(), "invalid name");
        assert_eq!(parse_person("name=Ada-1;age=20").unwrap_err(), "invalid name");
    }

    #[test]
    fn rejects_bad_structure_and_unknown_fields() {
        assert_eq!(parse_person("name=Ada;city=London;age=20").unwrap_err(), "unknown field");
        assert_eq!(parse_person("name=Ada;age").unwrap_err(), "invalid field");
    }
}
