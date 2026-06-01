use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub fields: HashMap<String, String>,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, String> {
    let mut records = Vec::new();

    for block in input.split("\n\n") {
        if block.trim().is_empty() {
            continue;
        }

        let mut fields = HashMap::new();
        for line in block.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let (key, value) = line
                .split_once(':')
                .ok_or_else(|| format!("invalid line: {line}"))?;

            fields.insert(key.trim().to_string(), value.trim().to_string());
        }

        if !fields.contains_key("id") {
            return Err("missing id".to_string());
        }

        records.push(Record { fields });
    }

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multiple_records() {
        let input = "id:100\nname:alpha\n\nid:200\nname:beta\nstatus:active";
        let records = parse_records(input).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].fields.get("id").unwrap(), "100");
        assert_eq!(records[1].fields.get("status").unwrap(), "active");
    }

    #[test]
    fn rejects_duplicate_keys_within_record() {
        let err = parse_records("id:100\nname:alpha\nname:beta").unwrap_err();
        assert!(err.contains("duplicate key: name"));
    }

    #[test]
    fn rejects_empty_required_values() {
        let err = parse_records("id:100\nname:   ").unwrap_err();
        assert_eq!(err, "empty required field: name");
    }

    #[test]
    fn rejects_missing_colon() {
        let err = parse_records("id:100\nname alpha").unwrap_err();
        assert!(err.contains("invalid line: name alpha"));
    }

    #[test]
    fn rejects_missing_name() {
        let err = parse_records("id:100\nstatus:active").unwrap_err();
        assert_eq!(err, "missing name");
    }
}
