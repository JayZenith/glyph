pub struct Record<'a> {
    pub label: &'a str,
    pub active: bool,
    pub score: i32,
}

pub fn collect_labels(records: &[Record<'_>], min_score: i32) -> Vec<String> {
    let mut labels: Vec<String> = records
        .iter()
        .filter(|r| r.score >= min_score)
        .map(|r| r.label.trim().to_string())
        .collect();
    labels.sort();
    labels.dedup();
    labels
}

#[cfg(test)]
mod tests {
    use super::{collect_labels, Record};

    #[test]
    fn keeps_only_active_nonempty_labels_and_sorts_unique() {
        let records = [
            Record { label: " beta ", active: true, score: 5 },
            Record { label: "", active: true, score: 9 },
            Record { label: "alpha", active: false, score: 10 },
            Record { label: "alpha", active: true, score: 10 },
            Record { label: "beta", active: true, score: 7 },
            Record { label: "gamma", active: true, score: 4 },
            Record { label: "   ", active: true, score: 8 },
        ];

        let got = collect_labels(&records, 5);
        assert_eq!(got, vec!["alpha", "beta"]);
    }

    #[test]
    fn allows_zero_threshold_but_still_filters_inactive_and_blank() {
        let records = [
            Record { label: "zeta", active: false, score: 0 },
            Record { label: " eta ", active: true, score: 0 },
            Record { label: "", active: true, score: 1 },
            Record { label: "theta", active: true, score: -1 },
        ];

        let got = collect_labels(&records, 0);
        assert_eq!(got, vec!["eta"]);
    }
}
