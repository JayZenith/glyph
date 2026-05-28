pub fn parse_record(line: &str) -> Result<(&str, u32), &'static str> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 2 {
        return Err("invalid record");
    }

    let name = parts[0];
    if name.is_empty() {
        return Err("invalid record");
    }

    let qty = parts[1].parse::<u32>().map_err(|_| "invalid record")?;
    Ok((name, qty))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("apples,12"), Ok(("apples", 12)));
    }

    #[test]
    fn rejects_missing_field() {
        assert_eq!(parse_record("apples"), Err("invalid record"));
    }

    #[test]
    fn rejects_extra_field() {
        assert_eq!(parse_record("apples,12,fresh"), Err("invalid record"));
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(parse_record(",12"), Err("invalid record"));
    }
}
