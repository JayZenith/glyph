#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub status: String,
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
        let mut status = String::new();
        let mut tags = Vec::new();

        for field in line.split(';') {
            let field = field.trim();
            if field.is_empty() {
                continue;
            }
            let (key, value) = field
                .split_once('=')
                .ok_or_else(|| format!("line {}: invalid field", line_no + 1))?;
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
                "status" => status = value.to_string(),
                "tags" => {
                    tags = value
                        .split(',')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect();
                }
                _ => return Err(format!("line {}: unknown key {}", line_no + 1, key)),
            }
        }

        let id = id.ok_or_else(|| format!("line {}: missing id", line_no + 1))?;
        out.push(Record { id, status, tags });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_lines_and_trims_tag_items() {
        let input = "id=7; status=active; tags=red, blue ,green\n\nid=8;status=paused;tags=solo";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record {
                    id: 7,
                    status: "active".to_string(),
                    tags: vec!["red".to_string(), "blue".to_string(), "green".to_string()],
                },
                Record {
                    id: 8,
                    status: "paused".to_string(),
                    tags: vec!["solo".to_string()],
                }
            ]
        );
    }

    #[test]
    fn rejects_missing_status() {
        let err = parse_records("id=2; tags=a,b").unwrap_err();
        assert_eq!(err, "line 1: missing status");
    }

    #[test]
    fn rejects_empty_status() {
        let err = parse_records("id=2; status= ; tags=a,b").unwrap_err();
        assert_eq!(err, "line 1: empty status");
    }

    #[test]
    fn rejects_empty_tag_entries() {
        let err = parse_records("id=3; status=active; tags=red,,blue").unwrap_err();
        assert_eq!(err, "line 1: empty tag");
    }
}
