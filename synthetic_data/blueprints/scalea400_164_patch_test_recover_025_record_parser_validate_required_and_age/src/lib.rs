#[derive(Debug, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub city: Option<String>,
}

pub fn parse_people(input: &str) -> Result<Vec<Person>, String> {
    let mut people = Vec::new();

    for (line_no, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let mut name = String::new();
        let mut age = 0u8;
        let mut city = None;

        for field in line.split(';') {
            let mut parts = field.splitn(2, '=');
            let key = parts.next().unwrap_or("").trim();
            let value = parts.next().unwrap_or("").trim();

            match key {
                "name" => name = value.to_string(),
                "age" => age = value.parse::<u8>().unwrap_or(0),
                "city" => city = Some(value.to_string()),
                _ => return Err(format!("line {}: unknown field", line_no + 1)),
            }
        }

        people.push(Person { name, age, city });
    }

    Ok(people)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records() {
        let input = "name=Ana;age=27;city=Oslo\nname=Bo;age=31";
        let people = parse_people(input).unwrap();
        assert_eq!(
            people,
            vec![
                Person {
                    name: "Ana".into(),
                    age: 27,
                    city: Some("Oslo".into())
                },
                Person {
                    name: "Bo".into(),
                    age: 31,
                    city: None
                }
            ]
        );
    }

    #[test]
    fn rejects_missing_required_fields() {
        let err = parse_people("name=Ana\nage=20").unwrap_err();
        assert_eq!(err, "line 1: missing required field");
    }

    #[test]
    fn rejects_malformed_and_negative_age() {
        let err = parse_people("name=Ana;age=-2").unwrap_err();
        assert_eq!(err, "line 1: invalid age");

        let err = parse_people("name=Ana;age=4;oops").unwrap_err();
        assert_eq!(err, "line 1: malformed field");
    }
}
