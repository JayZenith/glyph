use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkLog {
    pub department: &'static str,
    pub hours: u32,
    pub billable: bool,
}

pub fn billable_hours_by_department(logs: &[WorkLog]) -> Vec<(String, u32)> {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for log in logs.iter().filter(|log| log.billable) {
        *totals.entry(log.department).or_insert(0) += log.hours;
    }

    totals
        .into_iter()
        .map(|(dept, hours)| (dept.to_string(), hours))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{billable_hours_by_department, WorkLog};

    #[test]
    fn totals_only_billable_hours_grouped_by_department() {
        let logs = vec![
            WorkLog { department: "ops", hours: 5, billable: true },
            WorkLog { department: "sales", hours: 3, billable: false },
            WorkLog { department: "ops", hours: 2, billable: true },
            WorkLog { department: "sales", hours: 4, billable: true },
            WorkLog { department: "hr", hours: 7, billable: false },
        ];

        let summary = billable_hours_by_department(&logs);
        assert_eq!(summary, vec![
            ("ops".to_string(), 7),
            ("sales".to_string(), 4),
        ]);
    }

    #[test]
    fn returns_departments_in_sorted_order() {
        let logs = vec![
            WorkLog { department: "zeta", hours: 1, billable: true },
            WorkLog { department: "alpha", hours: 2, billable: true },
            WorkLog { department: "zeta", hours: 3, billable: true },
        ];

        let summary = billable_hours_by_department(&logs);
        assert_eq!(summary, vec![
            ("alpha".to_string(), 2),
            ("zeta".to_string(), 4),
        ]);
    }

    #[test]
    fn empty_when_no_billable_entries_exist() {
        let logs = vec![
            WorkLog { department: "ops", hours: 5, billable: false },
            WorkLog { department: "sales", hours: 3, billable: false },
        ];

        let summary = billable_hours_by_department(&logs);
        assert!(summary.is_empty());
    }
}
