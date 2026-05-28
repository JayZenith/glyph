#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub key: String,
    pub count: u32,
    pub active: bool,
}

pub fn parse_entries(input: &str) -> Result<Vec<Entry>, String> {
    let mut out = Vec::new();
    for (line_no, raw) in input.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut key = None;
        let mut count = None;
        let mut active = None;

        for part in line.split(';') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            let (name, value) = part
                .split_once('=')
                .ok_or_else(|| format!("line {}: missing '='", line_no + 1))?;
            match name.trim() {
                "key" => key = Some(value.trim().to_string()),
                "count" => {
                    count = Some(
                        value
                            .trim()
                            .parse::<u32>()
                            .map_err(|_| format!("line {}: invalid count", line_no + 1))?,
                    )
                }
                "active" => {
                    active = Some(match value.trim() {
                        "true" => true,
                        "false" => false,
                        _ => return Err(format!("line {}: invalid active", line_no + 1)),
                    })
                }
                _ => {}
            }
        }

        let key = key.ok_or_else(|| format!("line {}: missing key", line_no + 1))?;
        let count = count.ok_or_else(|| format!("line {}: missing count", line_no + 1))?;
        let active = active.ok_or_else(|| format!("line {}: missing active", line_no + 1))?;

        out.push(Entry { key, count, active });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_comments_and_valid_lines() {
        let input = "# config\nkey=alpha;count=3;active=true\n\nkey=beta; count=0 ; active=false\n";
        let got = parse_entries(input).unwrap();
        assert_eq!(
            got,
            vec![
                Entry {
                    key: "alpha".into(),
                    count: 3,
                    active: true,
                },
                Entry {
                    key: "beta".into(),
                    count: 0,
                    active: false,
                }
            ]
        );
    }

    #[test]
    fn rejects_duplicate_fields_and_unknown_fields() {
        let dup = parse_entries("key=a;count=1;count=2;active=true").unwrap_err();
        assert_eq!(dup, "line 1: duplicate count");

        let unknown = parse_entries("key=a;count=1;active=true;mode=fast").unwrap_err();
        assert_eq!(unknown, "line 1: unknown field mode");
    }

    #[test]
    fn validates_key_format() {
        let empty = parse_entries("key=;count=1;active=true").unwrap_err();
        assert_eq!(empty, "line 1: invalid key");

        let bad_upper = parse_entries("key=Abc;count=1;active=true").unwrap_err();
        assert_eq!(bad_upper, "line 1: invalid key");

        let bad_dash = parse_entries("key=a-b;count=1;active=true").unwrap_err();
        assert_eq!(bad_dash, "line 1: invalid key");
    }

    #[test]
    fn rejects_out_of_range_count() {
        let err = parse_entries("key=abc;count=1000;active=false").unwrap_err();
        assert_eq!(err, "line 1: invalid count");
    }
}
