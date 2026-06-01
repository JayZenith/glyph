#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub id: u32,
    pub ok: bool,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();
    for (idx, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 3 {
            return Err(format!("line {}: expected 3 fields", idx + 1));
        }

        let name = parts[0].to_string();
        let id = parts[1]
            .trim()
            .parse::<u32>()
            .map_err(|_| format!("line {}: invalid id", idx + 1))?;
        let ok = match parts[2].trim() {
            "ok" => true,
            "fail" => false,
            _ => return Err(format!("line {}: invalid status", idx + 1)),
        };

        out.push(Record { name, id, ok });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_rows_and_skips_blank_lines() {
        let input = "alice,12,ok\n\n bob ,7,FAIL\n";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    name: "alice".into(),
                    id: 12,
                    ok: true,
                },
                Record {
                    name: "bob".into(),
                    id: 7,
                    ok: false,
                },
            ]
        );
    }

    #[test]
    fn rejects_wrong_field_count() {
        let err = parse_records("a,1,ok,extra\n").unwrap_err();
        assert!(err.contains("expected 3 fields"));
    }

    #[test]
    fn rejects_blank_name_after_trim() {
        let err = parse_records("   ,4,ok\n").unwrap_err();
        assert!(err.contains("empty name"));
    }

    #[test]
    fn rejects_non_digit_id() {
        let err = parse_records("ann,12x,ok\n").unwrap_err();
        assert!(err.contains("invalid id"));
    }

    #[test]
    fn rejects_unknown_status() {
        let err = parse_records("ann,12,maybe\n").unwrap_err();
        assert!(err.contains("invalid status"));
    }
}
