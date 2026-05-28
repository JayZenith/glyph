#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub active: bool,
    pub age: Option<u8>,
    pub tags: Vec<String>,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 4 {
        return Err("expected 4 fields".into());
    }

    let id = parts[0].parse::<u32>().map_err(|_| "bad id")?;
    let active = match parts[1] {
        "true" => true,
        "false" => false,
        _ => return Err("bad active".into()),
    };

    let age = Some(parts[2].parse::<u8>().map_err(|_| "bad age")?);

    let tags = if parts[3].is_empty() {
        Vec::new()
    } else {
        parts[3].split('|').map(|s| s.to_string()).collect()
    };

    if tags.iter().any(|t| t.is_empty()) {
        return Err("empty tag".into());
    }

    Ok(Record { id, active, age, tags })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_record() {
        let r = parse_record("12,true,34,red|blue").unwrap();
        assert_eq!(r.id, 12);
        assert!(r.active);
        assert_eq!(r.age, Some(34));
        assert_eq!(r.tags, vec!["red".to_string(), "blue".to_string()]);
    }

    #[test]
    fn allows_missing_age() {
        let r = parse_record("7,false,,solo").unwrap();
        assert_eq!(r.age, None);
        assert_eq!(r.tags, vec!["solo".to_string()]);
    }

    #[test]
    fn rejects_blank_or_spaced_tags() {
        assert_eq!(parse_record("1,true,20,a||b"), Err("empty tag".into()));
        assert_eq!(parse_record("1,true,20,a| b"), Err("bad tag".into()));
        assert_eq!(parse_record("1,true,20,a|b "), Err("bad tag".into()));
    }

    #[test]
    fn rejects_non_ascii_alnum_tags() {
        assert_eq!(parse_record("1,true,20,ok|bad-tag"), Err("bad tag".into()));
    }
}
