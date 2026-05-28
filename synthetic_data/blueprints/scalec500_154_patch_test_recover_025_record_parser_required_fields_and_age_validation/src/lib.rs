#[derive(Debug, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub active: bool,
}

pub fn parse_person(input: &str) -> Result<Person, String> {
    let mut name = String::new();
    let mut age = 0u8;
    let mut active = false;

    for part in input.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("invalid field: {part}"))?;

        match key.trim() {
            "name" => name = value.trim().to_string(),
            "age" => {
                age = value
                    .trim()
                    .parse::<u8>()
                    .map_err(|_| "invalid age".to_string())?;
            }
            "active" => match value.trim() {
                "true" => active = true,
                "false" => active = false,
                _ => return Err("invalid active".to_string()),
            },
            _ => return Err(format!("unknown field: {}", key.trim())),
        }
    }

    Ok(Person { name, age, active })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record_with_spaces() {
        let p = parse_person(" name = Ada ; age = 42 ; active = true ").unwrap();
        assert_eq!(
            p,
            Person {
                name: "Ada".to_string(),
                age: 42,
                active: true,
            }
        );
    }

    #[test]
    fn rejects_missing_name() {
        assert_eq!(parse_person("age=22;active=false"), Err("missing name".to_string()));
    }

    #[test]
    fn rejects_age_out_of_range() {
        assert_eq!(parse_person("name=Eve;age=151;active=true"), Err("invalid age".to_string()));
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(parse_person("name=   ;age=20;active=true"), Err("missing name".to_string()));
    }

    #[test]
    fn rejects_duplicate_field() {
        assert_eq!(
            parse_person("name=Ada;age=20;name=Eve;active=true"),
            Err("duplicate field: name".to_string())
        );
    }
}
