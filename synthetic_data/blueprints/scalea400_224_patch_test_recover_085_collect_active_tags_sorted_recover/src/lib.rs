use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Record {
    pub active: bool,
    pub score: i32,
    pub tags: Vec<String>,
}

pub fn collect_tags(records: &[Record]) -> Vec<String> {
    let mut out: Vec<String> = records
        .iter()
        .flat_map(|r| r.tags.iter())
        .map(|tag| tag.trim().to_ascii_lowercase())
        .filter(|tag| !tag.is_empty())
        .collect();

    out.sort();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::{collect_tags, Record};

    fn s(values: &[&str]) -> Vec<String> {
        values.iter().map(|v| (*v).to_string()).collect()
    }

    #[test]
    fn keeps_only_active_records_and_positive_scores() {
        let records = vec![
            Record { active: true, score: 3, tags: s(&["alpha", "beta"]) },
            Record { active: false, score: 10, tags: s(&["hidden", "beta"]) },
            Record { active: true, score: 0, tags: s(&["zero"]) },
            Record { active: true, score: -2, tags: s(&["neg"]) },
        ];

        assert_eq!(collect_tags(&records), s(&["alpha", "beta"]));
    }

    #[test]
    fn trims_normalizes_sorts_and_dedups() {
        let records = vec![
            Record { active: true, score: 1, tags: s(&["  Zebra", "apple ", "APPLE", "mIxed"]) },
            Record { active: true, score: 2, tags: s(&["mixed", "banana", " ", "BANANA"]) },
        ];

        assert_eq!(collect_tags(&records), s(&["apple", "banana", "mixed", "zebra"]));
    }

    #[test]
    fn skips_tags_starting_with_x_after_normalization() {
        let records = vec![
            Record { active: true, score: 5, tags: s(&["xray", " Alpha ", "Xylophone", "beta"]) },
            Record { active: true, score: 1, tags: s(&["xerox", "gamma"]) },
        ];

        assert_eq!(collect_tags(&records), s(&["alpha", "beta", "gamma"]));
    }

    #[test]
    fn empty_result_when_nothing_qualifies() {
        let records = vec![
            Record { active: false, score: 3, tags: s(&["alpha"]) },
            Record { active: true, score: 0, tags: s(&["beta"]) },
            Record { active: true, score: 2, tags: s(&[" ", "xeno"]) },
        ];

        let empty: Vec<String> = Vec::new();
        assert_eq!(collect_tags(&records), empty);
    }
}
