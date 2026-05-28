#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub kind: String,
    pub active: bool,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();
    for (line_no, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let mut id = None;
        let mut kind = None;
        let mut active = None;

        for part in line.split(';') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            let Some((key, value)) = part.split_once('=') else {
                return Err(format!("line {}: bad field", line_no + 1));
            };
            match key.trim() {
                "id" => {
                    id = value.trim().parse::<u32>().ok();
                }
                "kind" => {
                    kind = Some(value.trim().to_string());
                }
                "active" => {
                    active = match value.trim() {
                        "true" => Some(true),
                        "false" => Some(false),
                        _ => None,
                    };
                }
                _ => {}
            }
        }

        let rec = Record {
            id: id.ok_or_else(|| format!("line {}: missing id", line_no + 1))?,
            kind: kind.ok_or_else(|| format!("line {}: missing kind", line_no + 1))?,
            active: active.ok_or_else(|| format!("line {}: missing active", line_no + 1))?,
        };
        out.push(rec);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records() {
        let input = "id=1;kind=alpha;active=true\nid=2;kind=beta;active=false";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record { id: 1, kind: "alpha".into(), active: true },
                Record { id: 2, kind: "beta".into(), active: false },
            ]
        );
    }

    #[test]
    fn rejects_unknown_field() {
        let err = parse_records("id=1;kind=alpha;active=true;extra=nope").unwrap_err();
        assert!(err.contains("unknown field"));
    }

    #[test]
    fn rejects_empty_kind() {
        let err = parse_records("id=1;kind= ;active=true").unwrap_err();
        assert!(err.contains("empty kind"));
    }

    #[test]
    fn rejects_duplicate_key() {
        let err = parse_records("id=1;kind=alpha;kind=beta;active=true").unwrap_err();
        assert!(err.contains("duplicate field"));
    }
}
