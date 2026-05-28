#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub qty: u32,
    pub enabled: bool,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();

    for (idx, raw) in input.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 3 {
            return Err(format!("line {}: expected 3 fields", idx + 1));
        }

        let name = parts[0].trim();
        if name.is_empty() {
            return Err(format!("line {}: empty name", idx + 1));
        }

        let qty: u32 = parts[1]
            .trim()
            .parse()
            .map_err(|_| format!("line {}: invalid qty", idx + 1))?;

        let enabled = match parts[2].trim() {
            "true" => true,
            "false" => false,
            _ => return Err(format!("line {}: invalid enabled", idx + 1)),
        };

        out.push(Record {
            name: name.to_string(),
            qty,
            enabled,
        });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records_and_skips_comments() {
        let input = "# inventory\napple | 10 | true\n\nbanana|2|false\n";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    name: "apple".into(),
                    qty: 10,
                    enabled: true,
                },
                Record {
                    name: "banana".into(),
                    qty: 2,
                    enabled: false,
                }
            ]
        );
    }

    #[test]
    fn rejects_extra_fields() {
        let err = parse_records("apple|10|true|extra").unwrap_err();
        assert_eq!(err, "line 1: expected 3 fields");
    }

    #[test]
    fn rejects_invalid_boolean() {
        let err = parse_records("apple|10|yes").unwrap_err();
        assert_eq!(err, "line 1: invalid enabled");
    }

    #[test]
    fn rejects_empty_name() {
        let err = parse_records(" |10|true").unwrap_err();
        assert_eq!(err, "line 1: empty name");
    }
}
