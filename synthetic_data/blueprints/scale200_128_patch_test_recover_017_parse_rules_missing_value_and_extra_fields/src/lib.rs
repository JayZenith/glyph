#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub id: u32,
    pub kind: String,
    pub active: bool,
}

pub fn parse_entry(input: &str) -> Result<Entry, String> {
    let mut id = None;
    let mut kind = None;
    let mut active = None;

    for part in input.split(',') {
        let mut it = part.split('=');
        let key = it.next().ok_or_else(|| "missing key".to_string())?.trim();
        let value = it.next().unwrap_or("").trim();

        match key {
            "id" => {
                id = Some(value.parse::<u32>().map_err(|_| "bad id".to_string())?);
            }
            "kind" => {
                kind = Some(value.to_string());
            }
            "active" => {
                active = Some(match value {
                    "true" => true,
                    "false" => false,
                    _ => return Err("bad active".to_string()),
                });
            }
            _ => return Err("unknown field".to_string()),
        }
    }

    Ok(Entry {
        id: id.ok_or_else(|| "missing id".to_string())?,
        kind: kind.ok_or_else(|| "missing kind".to_string())?,
        active: active.ok_or_else(|| "missing active".to_string())?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record() {
        assert_eq!(
            parse_entry("id=7,kind=alpha,active=true").unwrap(),
            Entry {
                id: 7,
                kind: "alpha".to_string(),
                active: true,
            }
        );
    }

    #[test]
    fn rejects_missing_kind_value() {
        assert!(parse_entry("id=7,kind=,active=true").is_err());
    }

    #[test]
    fn rejects_extra_equals_in_field() {
        assert!(parse_entry("id=7,kind=a=b,active=true").is_err());
    }
}
