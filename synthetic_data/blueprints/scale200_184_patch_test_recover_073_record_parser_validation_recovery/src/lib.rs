#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub kind: String,
    pub active: bool,
    pub tags: Vec<String>,
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
        let mut tags = Vec::new();

        for part in line.split(';') {
            let (key, value) = part
                .split_once('=')
                .ok_or_else(|| format!("line {}: bad field", line_no + 1))?;
            let key = key.trim();
            let value = value.trim();

            match key {
                "id" => {
                    id = Some(
                        value
                            .parse::<u32>()
                            .map_err(|_| format!("line {}: invalid id", line_no + 1))?,
                    );
                }
                "kind" => kind = Some(value.to_string()),
                "active" => match value {
                    "true" => active = Some(true),
                    "false" => active = Some(false),
                    _ => return Err(format!("line {}: invalid active", line_no + 1)),
                },
                "tags" => {
                    if !value.is_empty() {
                        tags = value.split(',').map(|s| s.to_string()).collect();
                    }
                }
                _ => return Err(format!("line {}: unknown field", line_no + 1)),
            }
        }

        let id = id.ok_or_else(|| format!("line {}: missing id", line_no + 1))?;
        let kind = kind.ok_or_else(|| format!("line {}: missing kind", line_no + 1))?;
        let active = active.ok_or_else(|| format!("line {}: missing active", line_no + 1))?;

        out.push(Record {
            id,
            kind,
            active,
            tags,
        });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records() {
        let input = "id=7;kind=alpha;active=true;tags=red,blue\nid=8;kind=beta;active=false;tags=";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    id: 7,
                    kind: "alpha".into(),
                    active: true,
                    tags: vec!["red".into(), "blue".into()],
                },
                Record {
                    id: 8,
                    kind: "beta".into(),
                    active: false,
                    tags: vec![],
                }
            ]
        );
    }

    #[test]
    fn trims_and_ignores_blank_lines() {
        let input = "  id=1; kind=test ; active=true ; tags=a,b  \n\n  \n id=2;kind=next;active=false;tags=x ";
        let got = parse_records(input).unwrap();
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].kind, "test");
        assert_eq!(got[0].tags, vec!["a", "b"]);
        assert_eq!(got[1].tags, vec!["x"]);
    }

    #[test]
    fn rejects_duplicate_fields() {
        let err = parse_records("id=1;kind=a;active=true;id=2").unwrap_err();
        assert!(err.contains("duplicate"));
    }

    #[test]
    fn rejects_empty_kind_and_blank_tags() {
        let err = parse_records("id=1;kind=;active=true;tags=ok").unwrap_err();
        assert!(err.contains("empty kind"));

        let err = parse_records("id=1;kind=a;active=true;tags=ok,,no").unwrap_err();
        assert!(err.contains("empty tag"));
    }

    #[test]
    fn rejects_tag_with_whitespace() {
        let err = parse_records("id=1;kind=a;active=true;tags=good,bad tag").unwrap_err();
        assert!(err.contains("invalid tag"));
    }
}
