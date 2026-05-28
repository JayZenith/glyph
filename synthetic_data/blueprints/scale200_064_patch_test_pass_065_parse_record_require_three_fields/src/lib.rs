pub fn parse_record(line: &str) -> Option<(&str, &str, &str)> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 3 {
        return None;
    }

    let id = parts[0];
    let kind = parts[1];
    let value = parts[2];

    if id.is_empty() || kind.is_empty() || value.is_empty() {
        return None;
    }

    Some((id, kind, value))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("42|temp|18"), Some(("42", "temp", "18")));
    }

    #[test]
    fn rejects_missing_field() {
        assert_eq!(parse_record("42|temp"), None);
    }

    #[test]
    fn rejects_empty_field() {
        assert_eq!(parse_record("42||18"), None);
    }

    #[test]
    fn rejects_extra_field() {
        assert_eq!(parse_record("42|temp|18|C"), None);
    }
}
