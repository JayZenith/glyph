pub fn extract_codes(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| {
            let (kind, value) = line.split_once(':')?;
            if kind != "code" {
                return None;
            }
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return None;
            }
            Some(trimmed.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::extract_codes;

    #[test]
    fn keeps_only_nonempty_code_entries() {
        let lines = ["code: AB12", "skip: nope", "code: ", "code:ZX9"];
        assert_eq!(extract_codes(&lines), vec!["AB12", "ZX9"]);
    }

    #[test]
    fn ignores_lowercase_and_strips_internal_spaces_from_codes() {
        let lines = ["code: a1", "code: A 1", "code: B  2", "code: C3"];
        assert_eq!(extract_codes(&lines), vec!["A1", "B2", "C3"]);
    }

    #[test]
    fn rejects_codes_without_any_ascii_digit() {
        let lines = ["code: ABC", "code: Z9", "code: Q R", "code: 7X"];
        assert_eq!(extract_codes(&lines), vec!["Z9", "7X"]);
    }
}
