pub fn parse_record(line: &str) -> Result<Vec<String>, String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;

    for ch in line.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                fields.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }

    if in_quotes {
        return Err("unterminated quote".into());
    }

    fields.push(cur.trim().to_string());
    Ok(fields)
}

pub fn validate_record(line: &str) -> Result<(u32, String, bool), String> {
    let fields = parse_record(line)?;
    if fields.len() != 3 {
        return Err("expected 3 fields".into());
    }

    let id: u32 = fields[0].parse().map_err(|_| "invalid id")?;
    let name = fields[1].clone();
    let active = match fields[2].as_str() {
        "true" => true,
        "false" => false,
        _ => return Err("invalid active flag".into()),
    };

    Ok((id, name, active))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quoted_commas() {
        let got = parse_record("7,\"alpha,beta\",true").unwrap();
        assert_eq!(got, vec!["7", "alpha,beta", "true"]);
    }

    #[test]
    fn preserves_spaces_inside_quoted_field() {
        let got = validate_record("5,\"  Ada Lovelace  \",false").unwrap();
        assert_eq!(got, (5, "  Ada Lovelace  ".to_string(), false));
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(validate_record("3,,true").unwrap_err(), "empty name");
    }

    #[test]
    fn rejects_whitespace_only_name() {
        assert_eq!(validate_record("3,   ,true").unwrap_err(), "empty name");
    }

    #[test]
    fn rejects_unquoted_extra_characters_after_quote() {
        assert_eq!(validate_record("4,\"abc\"xyz,false").unwrap_err(), "invalid quote usage");
    }

    #[test]
    fn rejects_zero_id() {
        assert_eq!(validate_record("0,bob,true").unwrap_err(), "invalid id");
    }
}
