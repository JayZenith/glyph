pub fn parse_record(line: &str) -> Result<(&str, u32, bool), &'static str> {
    let mut id = None;
    let mut qty = None;
    let mut active = None;

    for part in line.split(';') {
        let mut it = part.splitn(2, '=');
        let key = it.next().ok_or("missing key")?;
        let value = it.next().ok_or("missing value")?;

        match key {
            "id" => {
                if value.is_empty() || !value.chars().all(|c| c.is_ascii_alphanumeric()) {
                    return Err("bad id");
                }
                id = Some(value);
            }
            "qty" => {
                let n: u32 = value.parse().map_err(|_| "bad qty")?;
                if n == 0 {
                    return Err("bad qty");
                }
                qty = Some(n);
            }
            "active" => {
                active = Some(match value {
                    "true" => true,
                    "false" => false,
                    _ => return Err("bad active"),
                });
            }
            _ => return Err("unknown key"),
        }
    }

    match (id, qty, active) {
        (Some(id), Some(qty), Some(active)) => Ok((id, qty, active)),
        _ => Err("missing field"),
    }
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
        assert_eq!(parse_record("id=A12;qty=0;active=true"), Err("bad qty"));
    }

    #[test]
    fn rejects_unknown_key() {
        assert_eq!(parse_record("id=A12;qty=7;enabled=true"), Err("unknown key"));
    }

    #[test]
    fn rejects_missing_field() {
        assert_eq!(parse_record("id=A12;qty=7"), Err("missing field"));
    }

    #[test]
    fn rejects_invalid_id_chars() {
        assert_eq!(parse_record("id=A-12;qty=7;active=false"), Err("bad id"));
    }
}
