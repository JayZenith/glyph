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
            "active" => {
                active = Some(match value {
                    "true" => true,
                    "false" => false,
                    _ => return Err("bad active".to_string()),
                })
            }
            _ => return Err(format!("unknown field: {name}")),
        }
    }

    Ok(Record {
        key: key.ok_or_else(|| "missing key".to_string())?,
        count: count.ok_or_else(|| "missing count".to_string())?,
        active: active.ok_or_else(|| "missing active".to_string())?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        assert_eq!(
            parse_record("key=item_7;count=12;active=true").unwrap(),
            Record {
                key: "item_7".to_string(),
                count: 12,
                active: true,
            }
        );
    }

    #[test]
    fn rejects_empty_key() {
        assert_eq!(parse_record("key=;count=1;active=false"), Err("empty key".to_string()));
    }

    #[test]
    fn rejects_duplicate_field() {
        assert_eq!(
            parse_record("key=a;count=1;active=true;count=2"),
            Err("duplicate field: count".to_string())
        );
    }

    #[test]
    fn rejects_count_with_leading_zeros() {
        assert_eq!(
            parse_record("key=a;count=007;active=false"),
            Err("bad count".to_string())
        );
    }
}
