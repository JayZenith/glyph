#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub score: u8,
}

pub fn parse_record(input: &str) -> Result<Record, String> {
    let mut id = None;
    let mut name = None;
    let mut score = None;

    for part in input.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (key, value) = part.split_once('=').ok_or_else(|| "invalid field".to_string())?;
        let key = key.trim();
        let value = value.trim();

        match key {
            "id" => {
                if id.is_some() {
                    return Err("duplicate id".to_string());
                }
                id = Some(value.parse::<u32>().map_err(|_| "bad id".to_string())?);
            }
            "name" => {
                if name.is_some() {
                    return Err("duplicate name".to_string());
                }
                name = Some(value.to_string());
            }
            "score" => {
                if score.is_some() {
                    return Err("duplicate score".to_string());
                }
                score = Some(value.parse::<u8>().map_err(|_| "bad score".to_string())?);
            }
            _ => {}
        }
    }

    Ok(Record {
        id: id.ok_or_else(|| "missing id".to_string())?,
        name: name.ok_or_else(|| "missing name".to_string())?,
        score: score.ok_or_else(|| "missing score".to_string())?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_record_with_spacing() {
        let got = parse_record(" name = Ada Lovelace ; score = 99 ; id = 7 ").unwrap();
        assert_eq!(
            got,
            Record {
                id: 7,
                name: "Ada Lovelace".to_string(),
                score: 99,
            }
        );
    }

    #[test]
    fn rejects_unknown_key() {
        assert_eq!(parse_record("id=1;name=Ada;score=9;extra=yes"), Err("unknown key".to_string()));
    }

    #[test]
    fn rejects_blank_name_after_trim() {
        assert_eq!(parse_record("id=1;name=   ;score=9"), Err("blank name".to_string()));
    }

    #[test]
    fn rejects_score_above_100() {
        assert_eq!(parse_record("id=1;name=Ada;score=101"), Err("score out of range".to_string()));
    }

    #[test]
    fn rejects_duplicate_field() {
        assert_eq!(parse_record("id=1;name=Ada;score=9;score=10"), Err("duplicate score".to_string()));
    }
}
