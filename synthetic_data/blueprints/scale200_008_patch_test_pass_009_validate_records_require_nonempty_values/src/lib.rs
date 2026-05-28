pub fn parse_pairs(input: &str) -> Result<Vec<(&str, &str)>, String> {
    let mut out = Vec::new();
    for (idx, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("line {} missing '='", idx + 1))?;

        let key = key.trim();
        let value = value.trim();

        if key.is_empty() {
            return Err(format!("line {} has empty key", idx + 1));
        }

        out.push((key, value));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_pairs;

    #[test]
    fn parses_valid_lines_and_skips_blanks() {
        let input = "host = localhost\n\nport=8080\nmode = dev";
        let pairs = parse_pairs(input).unwrap();
        assert_eq!(pairs, vec![("host", "localhost"), ("port", "8080"), ("mode", "dev")]);
    }

    #[test]
    fn rejects_missing_separator() {
        let err = parse_pairs("host localhost").unwrap_err();
        assert_eq!(err, "line 1 missing '='");
    }

    #[test]
    fn rejects_empty_key() {
        let err = parse_pairs(" = value").unwrap_err();
        assert_eq!(err, "line 1 has empty key");
    }

    #[test]
    fn rejects_empty_value() {
        let err = parse_pairs("host =   ").unwrap_err();
        assert_eq!(err, "line 1 has empty value");
    }
}
