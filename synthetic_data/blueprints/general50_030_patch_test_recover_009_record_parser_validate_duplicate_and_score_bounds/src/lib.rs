use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub score: u8,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();

    for (idx, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 3 {
            return Err(format!("line {}: expected 3 fields", idx + 1));
        }

        let id: u32 = parts[0]
            .parse()
            .map_err(|_| format!("line {}: invalid id", idx + 1))?;
        if !seen.insert(id) {
            continue;
        }

        let name = parts[1].to_string();
        if name.is_empty() {
            return Err(format!("line {}: empty name", idx + 1));
        }

        let score: u8 = parts[2]
            .parse()
            .map_err(|_| format!("line {}: invalid score", idx + 1))?;

        out.push(Record { id, name, score });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_records() {
        let input = "1|Ada|90\n2|Bob|0\n3|Cy|100";
        let got = parse_records(input).unwrap();
        assert_eq!(
            got,
            vec![
                Record { id: 1, name: "Ada".into(), score: 90 },
                Record { id: 2, name: "Bob".into(), score: 0 },
                Record { id: 3, name: "Cy".into(), score: 100 },
            ]
        );
    }

    #[test]
    fn rejects_duplicate_ids() {
        let err = parse_records("1|Ada|10\n1|Bea|20").unwrap_err();
        assert_eq!(err, "line 2: duplicate id");
    }

    #[test]
    fn rejects_score_above_100() {
        let err = parse_records("7|Ada|101").unwrap_err();
        assert_eq!(err, "line 1: score out of range");
    }

    #[test]
    fn rejects_blank_name_after_trimming() {
        let err = parse_records("7|   |10").unwrap_err();
        assert_eq!(err, "line 1: empty name");
    }
}
