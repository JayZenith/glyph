#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub code: String,
    pub active: bool,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 3 {
        return Err("expected 3 fields".into());
    }

    let id: u32 = parts[0].parse().map_err(|_| "bad id")?;
    let code = parts[1].to_string();
    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("bad active".into()),
    };

    if code.is_empty() {
        return Err("empty code".into());
    }

    Ok(Record { id, code, active })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        assert_eq!(
            parse_record("7|AB12|true").unwrap(),
            Record {
                id: 7,
                code: "AB12".into(),
                active: true,
            }
        );
    }

    #[test]
    fn rejects_bad_field_count() {
        assert!(parse_record("7|AB12").is_err());
        assert!(parse_record("7|AB12|true|extra").is_err());
    }

    #[test]
    fn rejects_non_numeric_id() {
        assert!(parse_record("x|AB12|true").is_err());
    }

    #[test]
    fn rejects_non_boolean_active() {
        assert!(parse_record("7|AB12|yes").is_err());
    }

    #[test]
    fn rejects_code_with_lowercase_or_symbols() {
        assert!(parse_record("7|Ab12|true").is_err());
        assert!(parse_record("7|AB-12|true").is_err());
    }

    #[test]
    fn rejects_code_longer_than_six() {
        assert!(parse_record("7|ABC1234|false").is_err());
    }

    #[test]
    fn rejects_code_with_surrounding_spaces() {
        assert!(parse_record("7| AB12 |true").is_err());
    }
}
