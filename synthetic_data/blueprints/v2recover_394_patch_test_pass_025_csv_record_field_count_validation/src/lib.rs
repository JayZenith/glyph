pub fn parse_record(line: &str) -> Result<Vec<(&str, &str)>, &'static str> {
    let mut out = Vec::new();
    for part in line.split(',') {
        let (k, v) = part.split_once(':').ok_or("missing colon")?;
        if k.is_empty() || v.is_empty() {
            return Err("empty field");
        }
        out.push((k, v));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_three_fields() {
        let rec = parse_record("id:7,name:alice,role:admin").unwrap();
        assert_eq!(rec, vec![("id", "7"), ("name", "alice"), ("role", "admin")]);
    }

    #[test]
    fn rejects_missing_colon() {
        assert_eq!(parse_record("id:7,name"), Err("missing colon"));
    }

    #[test]
    fn rejects_empty_value() {
        assert_eq!(parse_record("id:7,name:"), Err("empty field"));
    }

    #[test]
    fn rejects_wrong_field_count() {
        assert_eq!(parse_record("id:7,name:alice"), Err("wrong field count"));
        assert_eq!(parse_record("id:7,name:alice,role:admin,extra:true"), Err("wrong field count"));
    }
}
