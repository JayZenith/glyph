pub fn parse_record(input: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();

    for raw in input.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        let Some((key, value)) = line.split_once(':') else {
            return Err(format!("invalid line: {line}"));
        };

        let key = key.trim();
        let value = value.trim();

        if key.is_empty() || value.is_empty() {
            return Err(format!("invalid line: {line}"));
        }

        out.push((key.to_string(), value.to_string()));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_basic_pairs() {
        let got = parse_record("name: widget\nversion: 1\n").unwrap();
        assert_eq!(
            got,
            vec![
                ("name".to_string(), "widget".to_string()),
                ("version".to_string(), "1".to_string())
            ]
        );
    }

    #[test]
    fn rejects_duplicate_keys() {
        let err = parse_record("name: widget\nname: gadget\n").unwrap_err();
        assert!(err.contains("duplicate key"), "{err}");
    }

    #[test]
    fn requires_name_field() {
        let err = parse_record("version: 1\n").unwrap_err();
        assert!(err.contains("missing required key: name"), "{err}");
    }

    #[test]
    fn rejects_extra_colons_in_line() {
        let err = parse_record("name: a:b\n").unwrap_err();
        assert!(err.contains("invalid line"), "{err}");
    }
}
