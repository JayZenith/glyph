use std::collections::BTreeMap;

pub fn parse_record(input: &str) -> Result<BTreeMap<String, String>, String> {
    let mut out = BTreeMap::new();

    for (idx, raw) in input.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key_raw, value_raw)) = line.split_once(':') else {
            return Err(format!("line {} missing ':'", idx + 1));
        };

        let key = key_raw.trim();
        let value = value_raw.trim();

        if key.is_empty() {
            return Err(format!("line {} empty key", idx + 1));
        }
        if value.is_empty() {
            return Err(format!("line {} empty value", idx + 1));
        }

        out.insert(key.to_string(), value.to_string());
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record_with_comments_and_spaces() {
        let input = "\n# header\n name : Alice \nage: 30\nactive: yes\n";
        let record = parse_record(input).unwrap();
        assert_eq!(record.get("name").map(String::as_str), Some("Alice"));
        assert_eq!(record.get("age").map(String::as_str), Some("30"));
        assert_eq!(record.get("active").map(String::as_str), Some("yes"));
    }

    #[test]
    fn rejects_unknown_key() {
        let err = parse_record("name: Alice\nrole: admin\nage: 30\n").unwrap_err();
        assert_eq!(err, "line 2 unknown key 'role'");
    }

    #[test]
    fn requires_name_and_age() {
        let err = parse_record("name: Alice\nactive: yes\n").unwrap_err();
        assert_eq!(err, "missing required key 'age'");
    }

    #[test]
    fn duplicate_key_reports_line_number() {
        let err = parse_record("name: Alice\nage: 30\nname: Bob\n").unwrap_err();
        assert_eq!(err, "line 3 duplicate key 'name'");
    }
}
