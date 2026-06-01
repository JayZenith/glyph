#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub active: bool,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();

    for block in input.split("\n\n") {
        if block.trim().is_empty() {
            continue;
        }

        let mut id = 0u32;
        let mut name = String::new();
        let mut active = false;

        for line in block.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let (key, value) = line
                .split_once(':')
                .ok_or_else(|| format!("invalid line: {}", line))?;

            match key.trim() {
                "id" => {
                    id = value.trim().parse().unwrap_or(0);
                }
                "name" => {
                    name = value.trim().to_string();
                }
                "active" => match value.trim() {
                    "true" => active = true,
                    "false" => active = false,
                    _ => return Err(format!("invalid active value: {}", value.trim())),
                },
                _ => return Err(format!("unknown field: {}", key.trim())),
            }
        }

        out.push(Record { id, name, active });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multiple_records() {
        let input = "id: 7\nname: Ada\nactive: true\n\nid: 8\nname: Bob\nactive: false\n";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    id: 7,
                    name: "Ada".to_string(),
                    active: true,
                },
                Record {
                    id: 8,
                    name: "Bob".to_string(),
                    active: false,
                }
            ]
        );
    }

    #[test]
    fn rejects_non_numeric_id() {
        let err = parse_records("id: x\nname: Ada\n").unwrap_err();
        assert!(err.contains("invalid id"));
    }

    #[test]
    fn rejects_zero_id() {
        let err = parse_records("id: 0\nname: Ada\n").unwrap_err();
        assert!(err.contains("invalid id"));
    }

    #[test]
    fn requires_name_field() {
        let err = parse_records("id: 4\nactive: true\n").unwrap_err();
        assert!(err.contains("missing required field"));
    }

    #[test]
    fn requires_id_field() {
        let err = parse_records("name: Ada\nactive: false\n").unwrap_err();
        assert!(err.contains("missing required field"));
    }

    #[test]
    fn rejects_duplicate_field() {
        let err = parse_records("id: 4\nid: 5\nname: Ada\n").unwrap_err();
        assert!(err.contains("duplicate field"));
    }
}
