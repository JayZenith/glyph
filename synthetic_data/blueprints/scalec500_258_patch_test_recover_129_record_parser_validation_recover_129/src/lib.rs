#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub id: u32,
    pub kind: String,
    pub tags: Vec<String>,
    pub active: bool,
}

pub fn parse_entries(input: &str) -> Result<Vec<Entry>, String> {
    let mut entries = Vec::new();

    for (line_no, raw) in input.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 4 {
            return Err(format!("line {}: expected 4 fields", line_no + 1));
        }

        let id: u32 = parts[0]
            .strip_prefix("id=")
            .ok_or_else(|| format!("line {}: missing id", line_no + 1))?
            .parse()
            .map_err(|_| format!("line {}: invalid id", line_no + 1))?;

        let kind = parts[1]
            .strip_prefix("kind=")
            .ok_or_else(|| format!("line {}: missing kind", line_no + 1))?
            .to_string();

        let tags_raw = parts[2]
            .strip_prefix("tags=")
            .ok_or_else(|| format!("line {}: missing tags", line_no + 1))?;
        let tags = if tags_raw.is_empty() {
            Vec::new()
        } else {
            tags_raw.split(',').map(|s| s.to_string()).collect()
        };

        let active = match parts[3]
            .strip_prefix("active=")
            .ok_or_else(|| format!("line {}: missing active", line_no + 1))?
        {
            "true" => true,
            "false" => false,
            _ => return Err(format!("line {}: invalid active", line_no + 1)),
        };

        if kind.is_empty() {
            return Err(format!("line {}: empty kind", line_no + 1));
        }

        entries.push(Entry {
            id,
            kind,
            tags,
            active,
        });
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records() {
        let input = "id=7|kind=system|tags=core,stable|active=true\nid=9|kind=user|tags=beta|active=false";
        let got = parse_entries(input).unwrap();
        assert_eq!(
            got,
            vec![
                Entry {
                    id: 7,
                    kind: "system".into(),
                    tags: vec!["core".into(), "stable".into()],
                    active: true,
                },
                Entry {
                    id: 9,
                    kind: "user".into(),
                    tags: vec!["beta".into()],
                    active: false,
                }
            ]
        );
    }

    #[test]
    fn rejects_whitespace_and_duplicate_tags() {
        let err = parse_entries("id=1|kind=ops|tags=ok, bad|active=true").unwrap_err();
        assert_eq!(err, "line 1: invalid tag");

        let err = parse_entries("id=1|kind=ops|tags=dup,dup|active=true").unwrap_err();
        assert_eq!(err, "line 1: duplicate tag");
    }

    #[test]
    fn rejects_zero_id_and_uppercase_kind() {
        let err = parse_entries("id=0|kind=ops|tags=a|active=true").unwrap_err();
        assert_eq!(err, "line 1: invalid id");

        let err = parse_entries("id=2|kind=Ops|tags=a|active=true").unwrap_err();
        assert_eq!(err, "line 1: invalid kind");
    }

    #[test]
    fn rejects_unknown_field_order_and_extra_field() {
        let err = parse_entries("kind=ops|id=2|tags=a|active=true").unwrap_err();
        assert_eq!(err, "line 1: missing id");

        let err = parse_entries("id=2|kind=ops|tags=a|active=true|note=x").unwrap_err();
        assert_eq!(err, "line 1: expected 4 fields");
    }
}
