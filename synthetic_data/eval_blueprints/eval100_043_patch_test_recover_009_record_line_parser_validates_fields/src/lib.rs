#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub active: bool,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 3 {
        return Err("expected 3 fields".into());
    }

    let id = parts[0].parse::<u32>().map_err(|_| "bad id")?;
    let name = parts[1].to_string();
    let active = parts[2].eq_ignore_ascii_case("active");

    Ok(Record { id, name, active })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_active_record() {
        assert_eq!(
            parse_record("7|Alice|active").unwrap(),
            Record {
                id: 7,
                name: "Alice".into(),
                active: true,
            }
        );
    }

    #[test]
    fn parses_valid_inactive_record() {
        assert_eq!(
            parse_record("8|Bob|inactive").unwrap(),
            Record {
                id: 8,
                name: "Bob".into(),
                active: false,
            }
        );
    }

    #[test]
    fn rejects_extra_field() {
        assert!(parse_record("7|Alice|active|admin").is_err());
    }

    #[test]
    fn rejects_zero_id() {
        assert!(parse_record("0|Alice|active").is_err());
    }

    #[test]
    fn rejects_blank_name() {
        assert!(parse_record("7||active").is_err());
    }

    #[test]
    fn rejects_unknown_status() {
        assert!(parse_record("7|Alice|paused").is_err());
    }
}
