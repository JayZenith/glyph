#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub key: String,
    pub count: u32,
    pub active: bool,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let mut key = None;
    let mut count = None;
    let mut active = None;

    for part in line.split(';') {
        let (name, value) = part
            .split_once('=')
            .ok_or_else(|| format!("invalid field: {part}"))?;

        match name {
            "key" => key = Some(value.to_string()),
            "count" => {
                count = Some(value.parse::<u32>().map_err(|_| "bad count".to_string())?)
            }
            "active" => active = Some(value == "true"),
            _ => return Err(format!("unknown field: {name}")),
        }
    }

    let key = key.ok_or_else(|| "missing key".to_string())?;
    let count = count.ok_or_else(|| "missing count".to_string())?;
    let active = active.ok_or_else(|| "missing active".to_string())?;

    if key.is_empty() {
        return Err("empty key".to_string());
    }

    Ok(Record { key, count, active })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        assert_eq!(
            parse_record("key=alpha_1;count=42;active=true").unwrap(),
            Record {
                key: "alpha_1".to_string(),
                count: 42,
                active: true,
            }
        );
    }

    #[test]
    fn rejects_duplicate_field() {
        assert!(parse_record("key=a;count=1;count=2;active=false").is_err());
    }

    #[test]
    fn rejects_invalid_key_chars() {
        assert!(parse_record("key=bad-key;count=1;active=true").is_err());
    }

    #[test]
    fn rejects_invalid_active_value() {
        assert!(parse_record("key=okay;count=1;active=yes").is_err());
    }
}
