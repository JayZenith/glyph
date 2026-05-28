pub fn parse_record(line: &str) -> Result<(&str, u32), &'static str> {
    let (name, count) = line.split_once(':').ok_or("missing separator")?;
    if name.is_empty() {
        return Err("empty name");
    }
    if count.is_empty() {
        return Err("empty count");
    }
    let value = count.parse::<u32>().map_err(|_| "invalid count")?;
    Ok((name, value))
}

pub fn is_valid_record(line: &str) -> bool {
    matches!(parse_record(line), Ok((name, value)) if value > 0 && name.bytes().all(|b| b.is_ascii_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::{is_valid_record, parse_record};

    #[test]
    fn parses_simple_record() {
        assert_eq!(parse_record("apples:12"), Ok(("apples", 12)));
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(parse_record("apples"), Err("missing separator"));
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(parse_record(":5"), Err("empty name"));
    }

    #[test]
    fn rejects_empty_count() {
        assert_eq!(parse_record("apples:"), Err("empty count"));
    }

    #[test]
    fn rejects_non_numeric_count() {
        assert_eq!(parse_record("apples:many"), Err("invalid count"));
    }

    #[test]
    fn valid_record_requires_lowercase_name_and_positive_count() {
        assert!(is_valid_record("pears:3"));
        assert!(!is_valid_record("Pears:3"));
        assert!(!is_valid_record("pears:0"));
    }
}
