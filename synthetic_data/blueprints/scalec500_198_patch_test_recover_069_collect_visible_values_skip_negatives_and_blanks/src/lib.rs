#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub visible: bool,
}

pub fn collect_visible_values(records: &[Record<'_>]) -> Vec<String> {
    records
        .iter()
        .filter(|r| r.visible)
        .map(|r| r.value.trim())
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_only_visible_nonblank_values() {
        let records = [
            Record { key: "a", value: " apple ", visible: true },
            Record { key: "b", value: "   ", visible: true },
            Record { key: "c", value: "pear", visible: false },
            Record { key: "d", value: " berry", visible: true },
        ];

        assert_eq!(collect_visible_values(&records), vec!["apple", "berry"]);
    }

    #[test]
    fn skips_values_marked_as_disabled_after_trimming() {
        let records = [
            Record { key: "a", value: "-draft", visible: true },
            Record { key: "b", value: " -hidden ", visible: true },
            Record { key: "c", value: "ready", visible: true },
            Record { key: "d", value: "", visible: true },
        ];

        assert_eq!(collect_visible_values(&records), vec!["ready"]);
    }
}
