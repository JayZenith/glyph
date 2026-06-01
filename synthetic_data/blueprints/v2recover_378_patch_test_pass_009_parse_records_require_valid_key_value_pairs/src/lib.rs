pub fn parse_record(line: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();

    for part in line.split(',') {
        let piece = part.trim();
        if piece.is_empty() {
            continue;
        }

        if let Some((key, value)) = piece.split_once('=') {
            out.push((key.trim().to_string(), value.trim().to_string()));
        } else {
            return Err(format!("invalid field: {piece}"));
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_simple_pairs() {
        let got = parse_record("name=alice, role=admin").unwrap();
        assert_eq!(
            got,
            vec![
                ("name".to_string(), "alice".to_string()),
                ("role".to_string(), "admin".to_string())
            ]
        );
    }

    #[test]
    fn rejects_missing_separator() {
        assert!(parse_record("name=alice, broken").is_err());
    }

    #[test]
    fn rejects_empty_key_or_value() {
        assert!(parse_record("=alice").is_err());
        assert!(parse_record("name=").is_err());
        assert!(parse_record("name=alice, role= ").is_err());
    }
}
