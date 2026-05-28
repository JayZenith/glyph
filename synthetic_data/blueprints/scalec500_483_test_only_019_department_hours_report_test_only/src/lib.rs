#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkLog {
    pub department: &'static str,
    pub hours: u32,
    pub billable: bool,
}

pub fn department_billable_hours(logs: &[WorkLog]) -> Vec<(String, u32)> {
    let mut pairs: Vec<(String, u32)> = Vec::new();

    for log in logs.iter().filter(|l| l.billable) {
        if let Some((_, total)) = pairs.iter_mut().find(|(dept, _)| dept == log.department) {
            *total += log.hours;
        } else {
            pairs.push((log.department.to_string(), log.hours));
        }
    }

    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs
}

pub fn total_billable_hours(logs: &[WorkLog]) -> u32 {
    logs.iter().filter(|l| l.billable).map(|l| l.hours).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_logs() -> Vec<WorkLog> {
        vec![
            WorkLog { department: "design", hours: 3, billable: true },
            WorkLog { department: "ops", hours: 5, billable: false },
            WorkLog { department: "design", hours: 4, billable: true },
            WorkLog { department: "engineering", hours: 6, billable: true },
            WorkLog { department: "ops", hours: 2, billable: true },
            WorkLog { department: "engineering", hours: 1, billable: false },
        ]
    }

    #[test]
    fn groups_only_billable_hours_per_department() {
        let report = department_billable_hours(&sample_logs());
        assert_eq!(
            report,
            vec![
                ("design".to_string(), 7),
                ("engineering".to_string(), 6),
                ("ops".to_string(), 2),
            ]
        );
    }

    #[test]
    fn sums_billable_hours_across_all_departments() {
        assert_eq!(total_billable_hours(&sample_logs()), 15);
    }

    #[test]
    fn empty_input_produces_empty_report_and_zero_total() {
        let logs = Vec::new();
        assert!(department_billable_hours(&logs).is_empty());
        assert_eq!(total_billable_hours(&logs), 0);
    }
}
