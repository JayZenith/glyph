pub fn parse_record(line: &str) -> Result<(&str, u32, bool), &'static str> {
    let mut id = None;
    let mut count = None;
    let mut active = None;

    for part in line.split(';') {
        let (key, value) = part.split_once('=').ok_or("missing equals")?;
        match key {
            "id" => {
                if value.is_empty() {
                    return Err("empty id");
                }
                id = Some(value);
            }
            "count" => {
                let n = value.parse::<u32>().map_err(|_| "bad count")?;
                count = Some(n);
            }
            "active" => match value {
                "true" => active = Some(true),
                "false" => active = Some(false),
                _ => return Err("bad active"),
            },
            _ => return Err("unknown field"),
        }
    }

    match (id, count, active) {
        (Some(id), Some(count), Some(active)) if count > 0 => Ok((id, count, active)),
        (Some(_), Some(0), Some(_)) => Err("count must be positive"),
        _ => Err("missing field"),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(
            parse_record("id=item-7;count=12;active=false"),
            Ok(("item-7", 12, false))
        );
    }

    #[test]
    fn rejects_zero_count() {
        assert_eq!(
            parse_record("id=item-7;count=0;active=true"),
            Err("count must be positive")
        );
    }

    #[test]
    fn rejects_unknown_field() {
        assert_eq!(
            parse_record("id=a;count=1;flag=yes;active=true"),
            Err("unknown field")
        );
    }

    #[test]
    fn rejects_missing_field() {
        assert_eq!(parse_record("id=a;active=true"), Err("missing field"));
    }

    #[test]
    fn rejects_missing_equals() {
        assert_eq!(parse_record("id=a;count=2;active"), Err("missing equals"));
    }
}
