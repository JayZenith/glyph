use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event<'a> {
    pub category: Option<&'a str>,
    pub count: u32,
}

pub fn build_report(events: &[Event<'_>]) -> Vec<String> {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for event in events {
        if let Some(category) = event.category {
            *totals.entry(category).or_insert(0) += event.count;
        }
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows
        .into_iter()
        .map(|(category, total)| format!("{}:{}", category, total))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_totals_and_places_unknown_last() {
        let events = [
            Event {
                category: Some("beta"),
                count: 3,
            },
            Event {
                category: None,
                count: 2,
            },
            Event {
                category: Some("alpha"),
                count: 4,
            },
            Event {
                category: Some("beta"),
                count: 1,
            },
            Event {
                category: None,
                count: 5,
            },
        ];

        assert_eq!(
            build_report(&events),
            vec!["alpha:4", "beta:4", "unknown:7"]
        );
    }

    #[test]
    fn omits_zero_total_categories_but_keeps_nonzero_unknown() {
        let events = [
            Event {
                category: Some("gamma"),
                count: 0,
            },
            Event {
                category: None,
                count: 1,
            },
            Event {
                category: Some("alpha"),
                count: 2,
            },
            Event {
                category: Some("gamma"),
                count: 0,
            },
        ];

        assert_eq!(build_report(&events), vec!["alpha:2", "unknown:1"]);
    }

    #[test]
    fn returns_empty_when_all_totals_are_zero() {
        let events = [
            Event {
                category: Some("alpha"),
                count: 0,
            },
            Event {
                category: None,
                count: 0,
            },
        ];

        let report = build_report(&events);
        assert!(report.is_empty());
    }
}
