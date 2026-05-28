#[derive(Debug, Clone)]
pub struct Record {
    pub active: bool,
    pub label: Option<String>,
}

pub fn collect_labels(records: &[Record]) -> Vec<String> {
    let mut labels: Vec<String> = records
        .iter()
        .filter_map(|r| r.label.clone())
        .collect();
    labels.sort();
    labels.dedup();
    labels
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(active: bool, label: Option<&str>) -> Record {
        Record {
            active,
            label: label.map(|s| s.to_string()),
        }
    }

    #[test]
    fn keeps_only_active_trimmed_non_blank_labels() {
        let records = vec![
            rec(true, Some("  beta  ")),
            rec(false, Some("alpha")),
            rec(true, None),
            rec(true, Some("   ")),
            rec(true, Some("gamma")),
        ];

        assert_eq!(collect_labels(&records), vec!["beta", "gamma"]);
    }

    #[test]
    fn dedups_case_insensitively_and_sorts_case_insensitively() {
        let records = vec![
            rec(true, Some("Zoo")),
            rec(true, Some(" apple ")),
            rec(true, Some("zoo")),
            rec(true, Some("Banana")),
            rec(true, Some("APPLE")),
            rec(true, Some("banana")),
        ];

        assert_eq!(collect_labels(&records), vec!["apple", "Banana", "Zoo"]);
    }
}
