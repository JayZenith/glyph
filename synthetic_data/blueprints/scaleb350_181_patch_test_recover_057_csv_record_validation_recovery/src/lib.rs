#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub age: u8,
    pub active: bool,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 3 {
        return Err("expected three fields".into());
    }

    let name = parts[0].to_string();
    let age = parts[1].parse::<u8>().map_err(|_| "invalid age")?;
    let active = parts[2].parse::<bool>().unwrap_or(false);

    Ok(Record { name, age, active })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record_with_trimmed_fields() {
        let rec = parse_record(" Alice |42| true ").unwrap();
        assert_eq!(rec, Record {
            name: "Alice".to_string(),
            age: 42,
            active: true,
        });
    }

    #[test]
    fn rejects_missing_or_extra_fields() {
        assert!(parse_record("Bob|20").is_err());
        assert!(parse_record("Bob|20|true|extra").is_err());
    }

    #[test]
    fn rejects_empty_name() {
        assert!(parse_record("   |20|false").is_err());
    }

    #[test]
    fn rejects_invalid_active_value() {
        assert!(parse_record("Bob|20|yes").is_err());
    }
}
