#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub key: String,
    pub count: u32,
    pub enabled: bool,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let mut key = None;
    let mut count = None;
    let mut enabled = None;

    for part in line.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (name, value) = part
            .split_once('=')
            .ok_or_else(|| format!("invalid field: {part}"))?;
        match name.trim() {
            "key" => key = Some(value.trim().to_string()),
            "count" => {
                let n = value.trim().parse::<u32>().map_err(|_| "bad count".to_string())?;
                count = Some(n);
            }
            "enabled" => enabled = Some(value.trim() == "true"),
            _ => return Err(format!("unknown field: {}", name.trim())),
        }
    }

    let key = key.ok_or_else(|| "missing key".to_string())?;
    let count = count.ok_or_else(|| "missing count".to_string())?;
    let enabled = enabled.ok_or_else(|| "missing enabled".to_string())?;

    Ok(Record { key, count, enabled })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record_with_spaces() {
        let rec = parse_record(" key = alpha_1 ; count = 7 ; enabled = false ").unwrap();
        assert_eq!(
            rec,
            Record {
                key: "alpha_1".into(),
                count: 7,
                enabled: false,
            }
        );
    }

    #[test]
    fn rejects_invalid_key_and_zero_count() {
        assert!(parse_record("key=bad-key;count=3;enabled=true").is_err());
        assert!(parse_record("key=good_9;count=0;enabled=true").is_err());
    }

    #[test]
    fn rejects_non_boolean_enabled() {
        assert!(parse_record("key=alpha;count=3;enabled=yes").is_err());
    }

    #[test]
    fn rejects_duplicate_fields() {
        assert!(parse_record("key=alpha;count=2;enabled=true;count=4").is_err());
    }
}
