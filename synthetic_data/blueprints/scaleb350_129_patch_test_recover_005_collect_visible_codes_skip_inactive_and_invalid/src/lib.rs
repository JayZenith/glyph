pub struct Record<'a> {
    pub active: bool,
    pub hidden: bool,
    pub code: Option<&'a str>,
}

pub fn visible_codes(records: &[Record<'_>]) -> Vec<String> {
    records
        .iter()
        .filter(|r| r.active)
        .filter_map(|r| r.code)
        .map(|code| code.to_ascii_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{visible_codes, Record};

    #[test]
    fn keeps_only_active_non_hidden_non_empty_codes() {
        let records = [
            Record { active: true, hidden: false, code: Some(" ab ") },
            Record { active: true, hidden: true, code: Some("hide") },
            Record { active: false, hidden: false, code: Some("off") },
            Record { active: true, hidden: false, code: Some("   ") },
            Record { active: true, hidden: false, code: None },
            Record { active: true, hidden: false, code: Some("x9") },
        ];

        assert_eq!(visible_codes(&records), vec!["AB", "X9"]);
    }

    #[test]
    fn preserves_order_after_filtering() {
        let records = [
            Record { active: true, hidden: false, code: Some(" b2") },
            Record { active: true, hidden: false, code: Some("a1 ") },
            Record { active: true, hidden: true, code: Some("z9") },
            Record { active: true, hidden: false, code: Some(" c3 ") },
        ];

        assert_eq!(visible_codes(&records), vec!["B2", "A1", "C3"]);
    }
}
