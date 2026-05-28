#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item<'a> {
    pub active: bool,
    pub code: &'a str,
}

pub fn collect_codes(items: &[Item<'_>]) -> Vec<String> {
    items
        .iter()
        .filter_map(|item| {
            if !item.active {
                return None;
            }

            let code = item.code.trim();
            if code.chars().all(|c| c.is_ascii_alphanumeric()) {
                Some(code.to_ascii_uppercase())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{collect_codes, Item};

    #[test]
    fn keeps_only_active_alnum_codes_and_normalizes_case() {
        let items = [
            Item { active: true, code: " ab12 " },
            Item { active: false, code: "zz9" },
            Item { active: true, code: "MiXed7" },
            Item { active: true, code: "bad-code" },
            Item { active: true, code: "ok99" },
        ];

        assert_eq!(collect_codes(&items), vec!["AB12", "MIXED7", "OK99"]);
    }

    #[test]
    fn ignores_blank_and_symbol_only_values_after_trimming() {
        let items = [
            Item { active: true, code: "   " },
            Item { active: true, code: "  a1  " },
            Item { active: true, code: "***" },
            Item { active: true, code: " z9 " },
        ];

        assert_eq!(collect_codes(&items), vec!["A1", "Z9"]);
    }
}
