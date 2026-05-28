#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub code: String,
    pub active: bool,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return Err("expected 3 fields".into());
    }

    let id = parts[0].trim().parse::<u32>().map_err(|_| "bad id")?;
    let code = parts[1].trim().to_string();
    let active = match parts[2].trim() {
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
    fn parses_valid_record_with_spaces() {
        let rec = parse_record("42, ABC-123 , true").unwrap();
        assert_eq!(
            rec,
            Record {
                id: 42,
                code: "ABC-123".into(),
                active: true,
            }
        );
    }

    #[test]
    fn rejects_extra_field() {
        assert_eq!(parse_record("1,ABC-123,true,extra"), Err("expected 3 fields".into()));
    }

    #[test]
    fn rejects_lowercase_code() {
        assert_eq!(parse_record("1,abc-123,true"), Err("bad code".into()));
    }

    #[test]
    fn rejects_code_without_dash() {
        assert_eq!(parse_record("1,ABC123,true"), Err("bad code".into()));
    }

    #[test]
    fn rejects_code_with_short_suffix() {
        assert_eq!(parse_record("1,ABC-12,true"), Err("bad code".into()));
    }

    #[test]
    fn rejects_non_boolean_active() {
        assert_eq!(parse_record("1,ABC-123,yes"), Err("bad active".into()));
    }
}
