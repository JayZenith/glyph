#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub age: u8,
    pub active: bool,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();
    for (line_no, raw) in input.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<_> = line.split('|').collect();
        if parts.len() < 3 {
            return Err(format!("line {}: expected 3 fields", line_no + 1));
        }

        let name = parts[0].trim().to_string();
        let age: u8 = parts[1]
            .trim()
            .parse()
            .map_err(|_| format!("line {}: invalid age", line_no + 1))?;
        let active = match parts[2].trim() {
            "active" => true,
            "inactive" => false,
            _ => return Err(format!("line {}: invalid status", line_no + 1)),
        };

        out.push(Record { name, age, active });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_rows() {
        let input = "Alice|34|active\nBob|0|inactive";
        let records = parse_records(input).unwrap();
        assert_eq!(
            records,
            vec![
                Record {
                    name: "Alice".into(),
                    age: 34,
                    active: true,
                },
                Record {
                    name: "Bob".into(),
                    age: 0,
                    active: false,
                }
            ]
        );
    }

    #[test]
    fn rejects_extra_fields() {
        let err = parse_records("Alice|34|active|extra").unwrap_err();
        assert_eq!(err, "line 1: expected 3 fields");
    }

    #[test]
    fn rejects_blank_name() {
        let err = parse_records("   |34|active").unwrap_err();
        assert_eq!(err, "line 1: blank name");
    }

    #[test]
    fn rejects_age_above_120() {
        let err = parse_records("Alice|121|active").unwrap_err();
        assert_eq!(err, "line 1: age out of range");
    }

    #[test]
    fn rejects_unknown_status() {
        let err = parse_records("Alice|34|busy").unwrap_err();
        assert_eq!(err, "line 1: invalid status");
    }
}
