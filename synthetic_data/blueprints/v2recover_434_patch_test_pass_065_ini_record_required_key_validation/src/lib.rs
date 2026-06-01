pub fn parse_record(input: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("invalid line: {line}"))?;

        let key = key.trim();
        let value = value.trim();

        if key.is_empty() {
            return Err("empty key".to_string());
        }

        out.push((key.to_string(), value.to_string()));
    }

    if out.iter().any(|(k, _)| k == "id") {
        Ok(out)
    } else {
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn accepts_comment_and_trimmed_pairs() {
        let parsed = parse_record("# header\n id = 42 \n name = Alice ").unwrap();
        assert_eq!(
            parsed,
            vec![
                ("id".to_string(), "42".to_string()),
                ("name".to_string(), "Alice".to_string())
            ]
        );
    }

    #[test]
    fn rejects_missing_required_id() {
        let err = parse_record("name=Alice\nrole=admin").unwrap_err();
        assert_eq!(err, "missing id");
    }

    #[test]
    fn rejects_line_without_separator() {
        let err = parse_record("id=7\nname").unwrap_err();
        assert_eq!(err, "invalid line: name");
    }
}
