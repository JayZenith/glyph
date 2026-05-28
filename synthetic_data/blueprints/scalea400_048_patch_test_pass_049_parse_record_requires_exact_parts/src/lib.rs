pub fn parse_record(line: &str) -> Result<(&str, u16), &'static str> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() < 2 {
        return Err("invalid format");
    }

    let name = parts[0];
    if name.is_empty() {
        return Err("empty name");
    }

    let port = parts[1].parse::<u16>().map_err(|_| "invalid port")?;
    Ok((name, port))
}

#[cfg(test)]
mod tests {
    use super::parse_record;

    #[test]
    fn parses_valid_record() {
        assert_eq!(parse_record("api:8080"), Ok(("api", 8080)));
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(parse_record("api8080"), Err("invalid format"));
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(parse_record(":8080"), Err("empty name"));
    }

    #[test]
    fn rejects_invalid_port() {
        assert_eq!(parse_record("api:eighty"), Err("invalid port"));
    }

    #[test]
    fn rejects_extra_fields() {
        assert_eq!(parse_record("api:8080:tcp"), Err("invalid format"));
    }
}
