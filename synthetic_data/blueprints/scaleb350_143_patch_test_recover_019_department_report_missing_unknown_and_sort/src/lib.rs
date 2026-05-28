use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub department: Option<&'static str>,
    pub hours: u32,
    pub billable: bool,
}

pub fn build_report(entries: &[Entry]) -> Vec<String> {
    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for entry in entries {
        if let Some(dept) = entry.department {
            let slot = totals.entry(dept).or_insert((0, 0));
            slot.0 += entry.hours;
            if entry.billable {
                slot.1 += entry.hours;
            }
        }
    }

    let mut lines: Vec<_> = totals
        .into_iter()
        .map(|(dept, (total, billable))| format!("{}: total={} billable={}", dept, total, billable))
        .collect();

    lines.sort();
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_hours_and_includes_unknown_department() {
        let entries = vec![
            Entry { department: Some("sales"), hours: 3, billable: true },
            Entry { department: None, hours: 2, billable: false },
            Entry { department: Some("sales"), hours: 4, billable: false },
            Entry { department: None, hours: 1, billable: true },
        ];

        assert_eq!(
            build_report(&entries),
            vec![
                "sales: total=7 billable=3",
                "unknown: total=3 billable=1",
            ]
        );
    }

    #[test]
    fn sorts_by_total_desc_then_department_name() {
        let entries = vec![
            Entry { department: Some("ops"), hours: 5, billable: true },
            Entry { department: Some("sales"), hours: 5, billable: false },
            Entry { department: Some("hr"), hours: 2, billable: true },
        ];

        assert_eq!(
            build_report(&entries),
            vec![
                "ops: total=5 billable=5",
                "sales: total=5 billable=0",
                "hr: total=2 billable=2",
            ]
        );
    }
}
