#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: String,
    pub qty: Option<u32>,
    pub note: Option<String>,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();
    for (line_idx, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let mut id = None;
        let mut qty = None;
        let mut note = None;

        for part in line.split(',') {
            let piece = part.trim();
            if piece.is_empty() {
                continue;
            }
            let Some((key, value)) = piece.split_once('=') else {
                return Err(format!("line {}: malformed field", line_idx + 1));
            };
            match key.trim() {
                "id" => id = Some(value.trim().to_string()),
                "qty" => {
                    let n = value.trim().parse::<u32>()
                        .map_err(|_| format!("line {}: invalid qty", line_idx + 1))?;
                    qty = Some(n);
                }
                "note" => note = Some(value.trim().to_string()),
                _ => {}
            }
        }

        out.push(Record {
            id: id.unwrap_or_default(),
            qty,
            note,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_lines() {
        let got = parse_records("id=a1, qty=3, note=ok\nid=b2").unwrap();
        assert_eq!(
            got,
            vec![
                Record { id: "a1".into(), qty: Some(3), note: Some("ok".into()) },
                Record { id: "b2".into(), qty: None, note: None },
            ]
        );
    }

    #[test]
    fn rejects_missing_id() {
        let err = parse_records("qty=4, note=test").unwrap_err();
        assert_eq!(err, "line 1: missing id");
    }

    #[test]
    fn rejects_zero_qty() {
        let err = parse_records("id=x, qty=0").unwrap_err();
        assert_eq!(err, "line 1: qty must be positive");
    }

    #[test]
    fn rejects_unknown_field() {
        let err = parse_records("id=x, color=red").unwrap_err();
        assert_eq!(err, "line 1: unknown field color");
    }

    #[test]
    fn rejects_malformed_field() {
        let err = parse_records("id=x, broken").unwrap_err();
        assert_eq!(err, "line 1: malformed field");
    }
}
