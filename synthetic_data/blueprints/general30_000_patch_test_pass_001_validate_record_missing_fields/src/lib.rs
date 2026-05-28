pub fn validate_record(line: &str) -> bool {
    let mut seen_id = false;
    let mut seen_name = false;

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
                seen_id = true;
            }
            "name" => seen_name = true,
            "active" => {
                if value != "true" && value != "false" {
                    return false;
                }
            }
            _ => return false,
        }
    }

    seen_id || seen_name
}

#[cfg(test)]
mod tests {
    use super::validate_record;

    #[test]
    fn accepts_complete_valid_record() {
        assert!(validate_record("id=42;name=alice;active=true"));
    }

    #[test]
    fn rejects_missing_name() {
        assert!(!validate_record("id=42;active=true"));
    }

    #[test]
    fn rejects_missing_id() {
        assert!(!validate_record("name=alice;active=false"));
    }

    #[test]
    fn rejects_bad_active_value() {
        assert!(!validate_record("id=42;name=alice;active=yes"));
    }

    #[test]
    fn rejects_unknown_field() {
        assert!(!validate_record("id=42;name=alice;role=admin"));
    }
}
