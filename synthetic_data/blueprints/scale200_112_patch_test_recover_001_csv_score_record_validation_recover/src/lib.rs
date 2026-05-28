#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub score: u8,
    pub active: bool,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 3 {
        return Err("expected 3 fields".into());
    }

    let name = parts[0].to_string();
    if name.is_empty() {
        return Err("name empty".into());
    }

    let score: u8 = parts[1].parse().map_err(|_| "invalid score")?;
    if score > 100 {
        return Err("score out of range".into());
    }

    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return Err("invalid active flag".into()),
    };

    Ok(Record { name, score, active })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let r = parse_record("alice,42,true").unwrap();
        assert_eq!(r, Record {
            name: "alice".to_string(),
            score: 42,
            active: true,
        });
    }

    #[test]
    fn rejects_wrong_field_count() {
        assert!(parse_record("alice,42").is_err());
        assert!(parse_record("alice,42,true,extra").is_err());
    }

    #[test]
    fn trims_name_and_flag() {
        let r = parse_record("  bob  ,7,false  ").unwrap();
        assert_eq!(r.name, "bob");
        assert!(!r.active);
    }

    #[test]
    fn rejects_blank_name_after_trimming() {
        assert!(parse_record("   ,9,true").is_err());
    }

    #[test]
    fn rejects_leading_zero_score_except_zero() {
        assert!(parse_record("amy,007,true").is_err());
        assert_eq!(parse_record("amy,0,true").unwrap().score, 0);
    }
}
