#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub active: bool,
}

pub fn parse_record(line: &str) -> Option<Record> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 3 {
        return None;
    }

    let id = parts[0].parse().ok()?;
    let name = parts[1];
    if name.is_empty() {
        return None;
    }

    let active = match parts[2] {
        "true" => true,
        "false" => false,
        _ => return None,
    };

    Some(Record {
        id,
        name: name.to_string(),
        active,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        assert_eq!(
            parse_record("12|Ada|true"),
            Some(Record {
                id: 12,
                name: "Ada".to_string(),
                active: true,
            })
        );
    }

    #[test]
    fn rejects_missing_name() {
        assert_eq!(parse_record("12||false"), None);
    }

    #[test]
    fn rejects_invalid_boolean() {
        assert_eq!(parse_record("12|Ada|yes"), None);
    }

    #[test]
    fn rejects_extra_fields() {
        assert_eq!(parse_record("12|Ada|true|admin"), None);
    }
}
