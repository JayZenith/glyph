#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub active: bool,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();

    for (line_no, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let mut id = None;
        let mut name = None;
        let mut active = false;

        for field in line.split(';') {
            let mut parts = field.splitn(2, '=');
            let key = parts.next().unwrap_or("").trim();
            let value = parts.next().unwrap_or("").trim();

            match key {
                "id" => {
                    id = value.parse::<u32>().ok();
                }
                "name" => {
                    name = Some(value.to_string());
                }
                "active" => {
                    active = value == "true";
                }
                _ => {}
            }
        }

        let id = id.ok_or_else(|| format!("line {}: missing id", line_no + 1))?;
        let name = name.ok_or_else(|| format!("line {}: missing name", line_no + 1))?;

        out.push(Record { id, name, active });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records_with_blank_lines() {
        let input = "\n id=1; name=Alice ; active=true\n\nname=Bob;id=2;active=false\n";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    id: 1,
                    name: "Alice".to_string(),
                    active: true,
                },
                Record {
                    id: 2,
                    name: "Bob".to_string(),
                    active: false,
                },
            ]
        );
    }

    #[test]
    fn rejects_unknown_keys() {
        let err = parse_records("id=1;name=Amy;role=admin;active=true").unwrap_err();
        assert_eq!(err, "line 1: unknown field role");
    }

    #[test]
    fn rejects_empty_name() {
        let err = parse_records("id=9;name=   ;active=true").unwrap_err();
        assert_eq!(err, "line 1: empty name");
    }

    #[test]
    fn rejects_invalid_active_value() {
        let err = parse_records("id=3;name=Cal;active=yes").unwrap_err();
        assert_eq!(err, "line 1: invalid active");
    }
}
