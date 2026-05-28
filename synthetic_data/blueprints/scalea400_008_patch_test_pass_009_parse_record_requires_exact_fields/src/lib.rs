#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub key: String,
    pub value: u32,
}

pub fn parse_record(line: &str) -> Result<Record, &'static str> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() < 2 {
        return Err("invalid format");
    }

    let key = parts[0];
    let value = parts[1].parse::<u32>().map_err(|_| "invalid value")?;

    if key.is_empty() {
        return Err("missing key");
    }

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
            parse_record("alpha:42").unwrap(),
            Record {
                key: "alpha".to_string(),
                value: 42,
            }
        );
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(parse_record("alpha").unwrap_err(), "invalid format");
    }

    #[test]
    fn rejects_extra_separator() {
        assert_eq!(parse_record("alpha:42:tail").unwrap_err(), "invalid format");
    }

    #[test]
    fn rejects_non_numeric_value() {
        assert_eq!(parse_record("alpha:xyz").unwrap_err(), "invalid value");
    }

    #[test]
    fn rejects_empty_key() {
        assert_eq!(parse_record(":5").unwrap_err(), "missing key");
    }
}
