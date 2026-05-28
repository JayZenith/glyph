pub fn parse_record(input: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("line {} missing separator", idx + 1))?;

        if key.is_empty() || value.is_empty() {
            return Err(format!("line {} has empty field", idx + 1));
        }

        out.push((key.to_string(), value.to_string()));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_lines() {
        let got = parse_record("name=alice\nrole=admin\nactive=true").unwrap();
        assert_eq!(
            got,
            vec![
                ("name".to_string(), "alice".to_string()),
                ("role".to_string(), "admin".to_string()),
                ("active".to_string(), "true".to_string()),
            ]
        );
    }

    #[test]
    fn rejects_missing_separator() {
        let err = parse_record("name=alice\ninvalid_line").unwrap_err();
        assert_eq!(err, "line 2 missing separator");
    }

    #[test]
    fn rejects_empty_field() {
        let err = parse_record("name=alice\nrole=").unwrap_err();
        assert_eq!(err, "line 2 has empty field");
    }

    #[test]
    fn rejects_multiple_separators() {
        let err = parse_record("name=alice=admin").unwrap_err();
        assert_eq!(err, "line 1 has too many separators");
    }
}
