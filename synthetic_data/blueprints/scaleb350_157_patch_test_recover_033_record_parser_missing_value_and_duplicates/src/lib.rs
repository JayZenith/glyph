pub fn parse_record(input: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();

    for part in input.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let (key, value) = part
            .split_once('=')
            .ok_or_else(|| format!("missing '=' in '{part}'"))?;

        let key = key.trim();
        let value = value.trim();

        if key.is_empty() {
            return Err("empty key".to_string());
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
        let got = parse_record("name=alice; role=admin").unwrap();
        assert_eq!(
            got,
            vec![
                ("name".to_string(), "alice".to_string()),
                ("role".to_string(), "admin".to_string())
            ]
        );
    }

    #[test]
    fn rejects_missing_value_after_trim() {
        let err = parse_record("name=alice; role=   ").unwrap_err();
        assert_eq!(err, "empty value for key 'role'");
    }

    #[test]
    fn rejects_duplicate_keys_even_with_spacing() {
        let err = parse_record("name=alice; role=admin; name = bob").unwrap_err();
        assert_eq!(err, "duplicate key 'name'");
    }

    #[test]
    fn ignores_trailing_separator() {
        let got = parse_record("name=alice; role=admin; ").unwrap();
        assert_eq!(got.len(), 2);
    }
}
