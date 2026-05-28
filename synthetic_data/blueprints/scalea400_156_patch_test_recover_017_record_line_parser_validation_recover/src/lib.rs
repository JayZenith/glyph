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
        let mut active = None;

        for field in line.split(';') {
            let mut parts = field.splitn(2, '=');
            let key = parts.next().unwrap_or("").trim();
            let value = parts.next().unwrap_or("").trim();

            match key {
                "id" => {
                    id = Some(value.parse::<u32>().map_err(|_| format!("line {}: bad id", line_no + 1))?);
                }
                "name" => {
                    name = Some(value.to_string());
                }
                "active" => {
                    active = Some(value == "true");
                }
                _ => {}
            }
        }

        let id = id.ok_or_else(|| format!("line {}: missing id", line_no + 1))?;
        let name = name.ok_or_else(|| format!("line {}: missing name", line_no + 1))?;
        let active = active.ok_or_else(|| format!("line {}: missing active", line_no + 1))?;

        out.push(Record { id, name, active });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records() {
        let input = "id=1;name=Alice;active=true\nid=2;name=Bob;active=false";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record { id: 1, name: "Alice".into(), active: true },
                Record { id: 2, name: "Bob".into(), active: false },
            ]
        );
    }

    #[test]
    fn rejects_unknown_field() {
        let err = parse_records("id=1;name=Ada;active=true;role=admin").unwrap_err();
        assert_eq!(err, "line 1: unknown field role");
    }

    #[test]
    fn rejects_invalid_active_value() {
        let err = parse_records("id=1;name=Ada;active=yes").unwrap_err();
        assert_eq!(err, "line 1: bad active");
    }

    #[test]
    fn rejects_empty_name() {
        let err = parse_records("id=7;name=   ;active=false").unwrap_err();
        assert_eq!(err, "line 1: empty name");
    }
}
