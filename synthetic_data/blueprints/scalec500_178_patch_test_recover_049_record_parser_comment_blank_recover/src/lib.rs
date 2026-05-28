#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub active: bool,
}

#[derive(Default)]
struct PartialPerson {
    name: Option<String>,
    age: Option<u8>,
    active: Option<bool>,
}

fn parse_bool(s: &str) -> Option<bool> {
    match s {
        "true" | "yes" | "1" => Some(true),
        "false" | "no" | "0" => Some(false),
        _ => None,
    }
}

fn finish_record(cur: &mut PartialPerson, out: &mut Vec<Person>) -> Result<(), String> {
    if cur.name.is_none() && cur.age.is_none() && cur.active.is_none() {
        return Ok(());
    }
    let person = Person {
        name: cur.name.take().ok_or_else(|| "missing name".to_string())?,
        age: cur.age.take().ok_or_else(|| "missing age".to_string())?,
        active: cur.active.take().ok_or_else(|| "missing active".to_string())?,
    };
    out.push(person);
    Ok(())
}

pub fn parse_people(input: &str) -> Result<Vec<Person>, String> {
    let mut out = Vec::new();
    let mut cur = PartialPerson::default();

    for raw in input.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("invalid line: {line}"))?;
        let key = key.trim();
        let value = value.trim();

        match key {
            "name" => cur.name = Some(value.to_string()),
            "age" => {
                let age = value.parse::<u8>().map_err(|_| format!("invalid age: {value}"))?;
                cur.age = Some(age);
            }
            "active" => {
                cur.active = Some(parse_bool(value).ok_or_else(|| format!("invalid active: {value}"))?);
            }
            _ => return Err(format!("unknown field: {key}")),
        }
    }

    finish_record(&mut cur, &mut out)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multiple_records_with_comments_and_blank_separators() {
        let input = "# team list
name=Ana
age=31
active=yes

# after separator
name=Ben
age=22
active=no
";
        let got = parse_people(input).unwrap();
        assert_eq!(
            got,
            vec![
                Person { name: "Ana".into(), age: 31, active: true },
                Person { name: "Ben".into(), age: 22, active: false },
            ]
        );
    }

    #[test]
    fn rejects_missing_field_before_separator() {
        let input = "name=Solo
age=40

name=Next
age=20
active=true
";
        assert_eq!(parse_people(input), Err("missing active".to_string()));
    }

    #[test]
    fn rejects_bad_age() {
        let input = "name=Bad
age=200x
active=true
";
        assert_eq!(parse_people(input), Err("invalid age: 200x".to_string()));
    }

    #[test]
    fn rejects_unknown_field() {
        let input = "name=Zed
age=9
role=admin
active=false
";
        assert_eq!(parse_people(input), Err("unknown field: role".to_string()));
    }
}
