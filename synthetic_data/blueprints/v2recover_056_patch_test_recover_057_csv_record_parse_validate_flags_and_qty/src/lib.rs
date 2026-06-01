#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: String,
    pub name: String,
    pub qty: u32,
    pub active: bool,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 4 {
        return Err("expected 4 fields".into());
    }

    let id = parts[0].to_string();
    let name = parts[1].to_string();
    let qty: u32 = parts[2].parse().map_err(|_| "invalid qty")?;
    let active = parts[3] == "active";

    Ok(Record { id, name, qty, active })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        let rec = parse_record("sku-1|Widget|12|active").unwrap();
        assert_eq!(
            rec,
            Record {
                id: "sku-1".into(),
                name: "Widget".into(),
                qty: 12,
                active: true,
            }
        );
    }

    #[test]
    fn rejects_wrong_field_count() {
        assert!(parse_record("a|b|3").is_err());
        assert!(parse_record("a|b|3|active|extra").is_err());
    }

    #[test]
    fn rejects_empty_text_fields() {
        assert!(parse_record("|Widget|3|active").is_err());
        assert!(parse_record("sku-1||3|active").is_err());
    }

    #[test]
    fn status_is_case_insensitive_but_must_be_known() {
        let rec = parse_record("sku-2|Gadget|0|INACTIVE").unwrap();
        assert!(!rec.active);
        assert!(parse_record("sku-2|Gadget|0|paused").is_err());
    }
}
