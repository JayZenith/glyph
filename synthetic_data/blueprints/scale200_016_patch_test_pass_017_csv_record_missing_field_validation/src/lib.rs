#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub key: String,
    pub value: i32,
}

pub fn parse_record(line: &str) -> Result<Record, &'static str> {
    let mut parts = line.split(':');
    let key = parts.next().ok_or("missing key")?;
    let value = parts.next().ok_or("missing value")?;

    if parts.next().is_some() {
        return Err("too many fields");
    }

    let value = value.parse::<i32>().map_err(|_| "invalid value")?;
    Ok(Record {
        key: key.to_string(),
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        assert_eq!(
            parse_record("apples:12"),
            Ok(Record {
                key: "apples".to_string(),
                value: 12
            })
        );
    }

    #[test]
    fn rejects_missing_value() {
        assert_eq!(parse_record("apples:"), Err("missing value"));
    }

    #[test]
    fn rejects_missing_key() {
        assert_eq!(parse_record(":12"), Err("missing key"));
    }

    #[test]
    fn rejects_too_many_fields() {
        assert_eq!(parse_record("apples:12:extra"), Err("too many fields"));
    }

    #[test]
    fn rejects_invalid_number() {
        assert_eq!(parse_record("apples:xyz"), Err("invalid value"));
    }
}
