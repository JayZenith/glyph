pub fn validate_record(line: &str) -> bool {
    let mut id = None;
    let mut name = None;
    let mut score = None;

    for part in line.split(';') {
        let Some((key, value)) = part.split_once('=') else {
            return false;
        };

        if value.is_empty() {
            return false;
        }

        match key {
            "id" => {
                if !value.chars().all(|c| c.is_ascii_digit()) {
                    return false;
                }
                id = Some(value);
            }
            "name" => {
                if !value.chars().all(|c| c.is_ascii_alphabetic()) {
                    return false;
                }
                name = Some(value);
            }
            "score" => {
                let Ok(n) = value.parse::<u8>() else {
                    return false;
                };
                if n > 100 {
                    return false;
                }
                score = Some(n);
            }
            _ => return false,
        }
    }

    id.is_some() && name.is_some()
}

#[cfg(test)]
mod tests {
    use super::validate_record;

    #[test]
    fn accepts_valid_record() {
        assert!(validate_record("id=42;name=Alice;score=90"));
    }

    #[test]
    fn rejects_missing_score() {
        assert!(!validate_record("id=42;name=Alice"));
    }

    #[test]
    fn rejects_non_numeric_id() {
        assert!(!validate_record("id=x7;name=Alice;score=90"));
    }

    #[test]
    fn rejects_out_of_range_score() {
        assert!(!validate_record("id=42;name=Alice;score=120"));
    }
}
