use std::collections::HashMap;

pub fn parse_records(input: &str) -> Result<HashMap<String, String>, String> {
    let mut map = HashMap::new();

    for (idx, line) in input.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = line
            .split_once(':')
            .ok_or_else(|| format!("line {} malformed", idx + 1))?;

        if key.is_empty() || value.is_empty() {
            return Err(format!("line {} malformed", idx + 1));
        }

        if map.insert(key.to_string(), value.to_string()).is_some() {
            return Err(format!("line {} duplicate key", idx + 1));
        }
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::parse_records;

    #[test]
    fn parses_trimmed_pairs_and_skips_comments_and_blank_lines() {
        let input = "\n  # comment\n user : alice \nrole: admin\n\n";
        let map = parse_records(input).unwrap();
        assert_eq!(map.get("user").map(String::as_str), Some("alice"));
        assert_eq!(map.get("role").map(String::as_str), Some("admin"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn rejects_duplicate_keys_after_trimming() {
        let input = "user:alice\n user : bob\n";
        let err = parse_records(input).unwrap_err();
        assert_eq!(err, "line 2 duplicate key");
    }

    #[test]
    fn rejects_missing_colon_or_blank_fields() {
        assert_eq!(parse_records("broken\n").unwrap_err(), "line 1 malformed");
        assert_eq!(parse_records(":value\n").unwrap_err(), "line 1 malformed");
        assert_eq!(parse_records("key:   \n").unwrap_err(), "line 1 malformed");
    }
}
