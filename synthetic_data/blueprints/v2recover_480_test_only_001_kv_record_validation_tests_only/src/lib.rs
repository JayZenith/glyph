pub fn parse_record(line: &str) -> Result<(&str, u32, bool), &'static str> {
    let mut id = None;
    let mut qty = None;
    let mut active = None;

    for part in line.split(';') {
        let (key, value) = part.split_once('=').ok_or("missing equals")?;
        match key {
            "id" => {
                if value.len() != 3 || !value.chars().all(|c| c.is_ascii_uppercase()) {
                    return Err("bad id");
                }
                id = Some(value);
            }
            "qty" => {
                let n = value.parse::<u32>().map_err(|_| "bad qty")?;
                if n == 0 {
                    return Err("bad qty");
                }
                qty = Some(n);
            }
            "active" => match value {
                "true" => active = Some(true),
                "false" => active = Some(false),
                _ => return Err("bad active"),
            },
            _ => return Err("unknown key"),
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
        assert_eq!(parse_record("id=ABC;qty=7;active=true"), Ok(("ABC", 7, true)));
    }

    #[test]
    fn rejects_missing_field() {
        assert_eq!(parse_record("id=ABC;qty=7"), Err("missing active"));
    }

    #[test]
    fn rejects_bad_id_format() {
        assert_eq!(parse_record("id=AbC;qty=7;active=false"), Err("bad id"));
    }

    #[test]
    fn rejects_zero_qty() {
        assert_eq!(parse_record("id=XYZ;qty=0;active=false"), Err("bad qty"));
    }

    #[test]
    fn rejects_unknown_key() {
        assert_eq!(parse_record("id=XYZ;qty=2;flag=true"), Err("unknown key"));
    }

    #[test]
    fn rejects_missing_equals() {
        assert_eq!(parse_record("id=XYZ;qty=2;active"), Err("missing equals"));
    }
}
