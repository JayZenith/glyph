#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub active: bool,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 3 {
        return Err("expected 3 fields".to_string());
    }

    let id = parts[0].parse::<u32>().map_err(|_| "invalid id".to_string())?;
    let name = parts[1].trim().to_string();
    let active = match parts[2].trim() {
        "true" => true,
        "false" => false,
        _ => return Err("invalid active flag".to_string()),
    };

    Ok(Record { id, name, active })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        assert_eq!(
            parse_record("42|Ada|true").unwrap(),
            Record {
                id: 42,
                name: "Ada".to_string(),
                active: true,
            }
        );
    }

    #[test]
    fn rejects_extra_field() {
        assert_eq!(
            parse_record("42|Ada|true|extra").unwrap_err(),
            "expected 3 fields"
        );
    }

    #[test]
    fn rejects_blank_name() {
        assert_eq!(parse_record("42|   |false").unwrap_err(), "missing name");
    }

    #[test]
    fn rejects_invalid_bool() {
        assert_eq!(parse_record("42|Ada|yes").unwrap_err(), "invalid active flag");
    }
}
