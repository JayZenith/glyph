pub fn parse_record(line: &str) -> Result<(&str, u32, bool), &'static str> {
    let mut id = None;
    let mut qty = None;
    let mut active = None;

    for part in line.split(';') {
        let mut kv = part.splitn(2, '=');
        let key = kv.next().ok_or("missing key")?;
        let value = kv.next().ok_or("missing value")?;
        match key {
            "id" => {
                if value.is_empty() || !value.chars().all(|c| c.is_ascii_alphanumeric()) {
                    return Err("invalid id");
                }
                id = Some(value);
            }
            "qty" => {
                let n = value.parse::<u32>().map_err(|_| "invalid qty")?;
                if n == 0 {
                    return Err("invalid qty");
                }
                qty = Some(n);
            }
            "active" => match value {
                "true" => active = Some(true),
                "false" => active = Some(false),
                _ => return Err("invalid active"),
            },
            _ => return Err("unknown field"),
        }
    }

    Ok((
        id.ok_or("missing id")?,
        qty.ok_or("missing qty")?,
        active.ok_or("missing active")?,
    ))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("id=A12;qty=7;active=true"), Ok(("A12", 7, true)));
    }

    #[test]
    fn rejects_zero_quantity() {
        assert_eq!(parse_record("id=A12;qty=0;active=true"), Err("invalid qty"));
    }

    #[test]
    fn rejects_unknown_field() {
        assert_eq!(parse_record("id=A12;qty=3;flag=yes"), Err("unknown field"));
    }

    #[test]
    fn rejects_non_alphanumeric_id() {
        assert_eq!(parse_record("id=A-12;qty=3;active=false"), Err("invalid id"));
    }

    #[test]
    fn reports_missing_required_field() {
        assert_eq!(parse_record("id=A12;active=false"), Err("missing qty"));
    }
}
