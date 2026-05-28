use std::collections::HashMap;

pub fn parse_records(input: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let mut out = Vec::new();

    for (line_no, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let mut record = HashMap::new();
        for part in line.split(',') {
            let piece = part.trim();
            let (key, value) = piece
                .split_once('=')
                .ok_or_else(|| format!("line {}: bad field", line_no + 1))?;
            let key = key.trim();
            let value = value.trim();
            if key.is_empty() || value.is_empty() {
                return Err(format!("line {}: empty key or value", line_no + 1));
            }
            record.insert(key.to_string(), value.to_string());
        }

        if !record.contains_key("id") || !record.contains_key("qty") {
            return Err(format!("line {}: missing required field", line_no + 1));
        }

        out.push(record);
    }

    Ok(out)
}

pub fn validate_records(records: &[HashMap<String, String>]) -> Result<(), String> {
    for (idx, rec) in records.iter().enumerate() {
        rec.get("id")
            .ok_or_else(|| format!("record {}: missing id", idx + 1))?
            .parse::<u32>()
            .map_err(|_| format!("record {}: invalid id", idx + 1))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_records, validate_records};

    #[test]
    fn accepts_valid_rows() {
        let input = "id=10,qty=3,name=widget\nid=11, qty=0, name=gadget";
        let records = parse_records(input).unwrap();
        validate_records(&records).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].get("name").unwrap(), "widget");
    }

    #[test]
    fn rejects_unknown_field_during_parse() {
        let err = parse_records("id=10,qty=3,color=red").unwrap_err();
        assert!(err.contains("unknown field"));
    }

    #[test]
    fn rejects_negative_qty_during_validation() {
        let records = parse_records("id=10,qty=-2,name=widget").unwrap();
        let err = validate_records(&records).unwrap_err();
        assert!(err.contains("invalid qty"));
    }

    #[test]
    fn rejects_non_numeric_qty_during_validation() {
        let records = parse_records("id=10,qty=abc").unwrap();
        let err = validate_records(&records).unwrap_err();
        assert!(err.contains("invalid qty"));
    }
}
