pub fn parse_record(line: &str) -> Result<(&str, &str), &'static str> {
    let line = line.trim();
    if line.is_empty() {
        return Err("empty line");
    }

    let (key, value) = line.split_once('=').ok_or("missing separator")?;
    if key.is_empty() || value.is_empty() {
        return Err("missing field");
    }
    if !key.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
        return Err("invalid key");
    }
    Ok((key, value))
}

pub fn validate_records(input: &str) -> Result<Vec<(&str, &str)>, &'static str> {
    input.lines().map(parse_record).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records() {
        let got = validate_records("name=alice\nrole=admin_user").unwrap();
        assert_eq!(got, vec![("name", "alice"), ("role", "admin_user")]);
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(parse_record("name"), Err("missing separator"));
    }

    #[test]
    fn rejects_extra_separator() {
        assert_eq!(parse_record("name=alice=admin"), Err("missing separator"));
    }

    #[test]
    fn rejects_invalid_key() {
        assert_eq!(parse_record("Name=alice"), Err("invalid key"));
    }
}
