#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub age: u8,
    pub active: bool,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();

    for (line_no, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let mut name: Option<String> = None;
        let mut age: Option<u8> = None;
        let mut active = false;

        for field in line.split(';') {
            let field = field.trim();
            if field.is_empty() {
                continue;
            }

            let Some((key, value)) = field.split_once('=') else {
                return Err(format!("line {}: malformed field", line_no + 1));
            };

            match key.trim() {
                "name" => name = Some(value.trim().to_string()),
                "age" => {
                    let n: u8 = value
                        .trim()
                        .parse()
                        .map_err(|_| format!("line {}: invalid age", line_no + 1))?;
                    age = Some(n);
                }
                "active" => active = value.trim() == "true",
                _ => return Err(format!("line {}: unknown key", line_no + 1)),
            }
        }

        let name = name.ok_or_else(|| format!("line {}: missing name", line_no + 1))?;
        let age = age.ok_or_else(|| format!("line {}: missing age", line_no + 1))?;

        out.push(Record { name, age, active });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records_with_spaces() {
        let input = " name = Ada ; age = 37 ; active = true \nname=Bob;age=0;active=false\n";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    name: "Ada".to_string(),
                    age: 37,
                    active: true,
                },
                Record {
                    name: "Bob".to_string(),
                    age: 0,
                    active: false,
                }
            ]
        );
    }

    #[test]
    fn rejects_empty_name_and_bad_boolean() {
        assert!(parse_records("name= ; age=9; active=true").is_err());
        assert!(parse_records("name=Ada; age=9; active=yes").is_err());
    }

    #[test]
    fn rejects_out_of_range_age_and_duplicate_keys() {
        assert!(parse_records("name=Ada; age=131; active=true").is_err());
        assert!(parse_records("name=Ada; age=5; age=6; active=true").is_err());
        assert!(parse_records("name=Ada; age=5; active=false; active=true").is_err());
    }

    #[test]
    fn rejects_malformed_or_unknown_fields() {
        assert!(parse_records("name=Ada; age; active=true").is_err());
        assert!(parse_records("name=Ada; age=5; role=admin; active=true").is_err());
    }
}
