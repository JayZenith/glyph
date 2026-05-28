#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub name: String,
    pub count: u32,
    pub active: bool,
}

pub fn parse_record(line: &str) -> Result<Record, String> {
    let mut parts = line.split('|');

    let name = parts.next().ok_or("missing name")?.trim().to_string();
    let count_str = parts.next().ok_or("missing count")?.trim();
    let active_str = parts.next().ok_or("missing active")?.trim();

    let count = count_str.parse::<u32>().map_err(|_| "bad count".to_string())?;
    let active = match active_str {
        "true" => true,
        "false" => false,
        _ => return Err("bad active".to_string()),
    };

    Ok(Record { name, count, active })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        assert_eq!(
            parse_record("alice|12|true").unwrap(),
            Record {
                name: "alice".to_string(),
                count: 12,
                active: true,
            }
        );
    }

    #[test]
    fn trims_name_and_bool_fields() {
        assert_eq!(
            parse_record("  bob smith  |7| false ").unwrap(),
            Record {
                name: "bob smith".to_string(),
                count: 7,
                active: false,
            }
        );
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(parse_record("   |5|true"), Err("empty name".to_string()));
    }

    #[test]
    fn rejects_extra_field() {
        assert_eq!(
            parse_record("cara|3|true|extra"),
            Err("wrong field count".to_string())
        );
    }

    #[test]
    fn rejects_missing_field() {
        assert_eq!(parse_record("dave|9"), Err("wrong field count".to_string()));
    }

    #[test]
    fn rejects_non_numeric_count() {
        assert_eq!(parse_record("eve|x|true"), Err("bad count".to_string()));
    }
}
