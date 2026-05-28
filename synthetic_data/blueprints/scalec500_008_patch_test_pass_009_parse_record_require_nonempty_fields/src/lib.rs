pub fn parse_record(input: &str) -> Result<Vec<(&str, &str)>, &'static str> {
    let mut out = Vec::new();

    for part in input.split(';') {
        if part.is_empty() {
            continue;
        }

        let (key, value) = part.split_once('=').ok_or("missing equals")?;
        out.push((key, value));
    }

    if out.is_empty() {
        return Err("empty record");
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_pairs() {
        let items = parse_record("name=alice;role=admin").unwrap();
        assert_eq!(items, vec![("name", "alice"), ("role", "admin")]);
    }

    #[test]
    fn rejects_missing_equals() {
        assert_eq!(parse_record("name=alice;broken").unwrap_err(), "missing equals");
    }

    #[test]
    fn rejects_empty_input() {
        assert_eq!(parse_record("").unwrap_err(), "empty record");
    }

    #[test]
    fn rejects_empty_key() {
        assert_eq!(parse_record("=alice").unwrap_err(), "empty field");
    }

    #[test]
    fn rejects_empty_value() {
        assert_eq!(parse_record("name=").unwrap_err(), "empty field");
    }
}
