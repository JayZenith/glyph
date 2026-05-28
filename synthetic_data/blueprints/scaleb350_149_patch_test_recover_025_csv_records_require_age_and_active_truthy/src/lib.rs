#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub active: bool,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();

    for (idx, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() < 4 {
            return Err(format!("line {}: expected 4 fields", idx + 1));
        }

        let id = parts[0]
            .parse::<u32>()
            .map_err(|_| format!("line {}: invalid id", idx + 1))?;

        let name = parts[1].to_string();
        if name.is_empty() {
            return Err(format!("line {}: empty name", idx + 1));
        }

        let age = parts[2].parse::<u8>().unwrap_or(0);

        let active = match parts[3] {
            "true" | "yes" | "1" => true,
            _ => false,
        };

        out.push(Record {
            id,
            name,
            age,
            active,
        });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_rows_with_trimming() {
        let input = "1, Ada , 37 , yes\n2,Bob,8,false";
        let records = parse_records(input).unwrap();
        assert_eq!(
            records,
            vec![
                Record {
                    id: 1,
                    name: "Ada".to_string(),
                    age: 37,
                    active: true,
                },
                Record {
                    id: 2,
                    name: "Bob".to_string(),
                    age: 8,
                    active: false,
                }
            ]
        );
    }

    #[test]
    fn rejects_invalid_age_instead_of_defaulting() {
        let err = parse_records("5,Eve,old,true").unwrap_err();
        assert_eq!(err, "line 1: invalid age");
    }

    #[test]
    fn rejects_unknown_active_value() {
        let err = parse_records("7,Neo,44,maybe").unwrap_err();
        assert_eq!(err, "line 1: invalid active");
    }

    #[test]
    fn rejects_extra_fields() {
        let err = parse_records("9,Max,30,true,extra").unwrap_err();
        assert_eq!(err, "line 1: expected 4 fields");
    }
}
