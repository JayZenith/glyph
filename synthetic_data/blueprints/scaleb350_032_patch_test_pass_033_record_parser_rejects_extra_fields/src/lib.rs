pub fn parse_record(line: &str) -> Option<(&str, u32, &str)> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 3 {
        return None;
    }

    let name = parts[0];
    if name.is_empty() {
        return None;
    }

    let age: u32 = parts[1].parse().ok()?;
    if age == 0 {
        return None;
    }

    let role = parts[2];
    match role {
        "admin" | "user" => Some((name, age, role)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("alice|34|admin"), Some(("alice", 34, "admin")));
    }

    #[test]
    fn rejects_bad_role() {
        assert_eq!(parse_record("alice|34|guest"), None);
    }

    #[test]
    fn rejects_zero_age() {
        assert_eq!(parse_record("alice|0|user"), None);
    }

    #[test]
    fn rejects_missing_name() {
        assert_eq!(parse_record("|22|user"), None);
    }

    #[test]
    fn rejects_extra_fields() {
        assert_eq!(parse_record("alice|34|admin|ops"), None);
    }
}
