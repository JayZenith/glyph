use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkLog {
    pub department: &'static str,
    pub employee: &'static str,
    pub hours: u32,
    pub billable: bool,
}

pub fn build_department_report(logs: &[WorkLog]) -> String {
    let mut grouped: BTreeMap<&str, (u32, u32, u32)> = BTreeMap::new();

    for log in logs {
        let entry = grouped.entry(log.department).or_insert((0, 0, 0));
        entry.0 += 1;
        entry.1 += log.hours;
        if log.billable {
            entry.2 += log.hours;
        }
    }

    let mut out = String::new();
    for (dept, (people, total, billable)) in grouped {
        let non_billable = total - billable;
        out.push_str(&format!("{}: employees={}, total={}, billable={}, non_billable={}\n", dept, people, total, billable, non_billable));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_groups_departments_and_sums_hours() {
        let logs = [
            WorkLog { department: "Design", employee: "Ava", hours: 5, billable: true },
            WorkLog { department: "Design", employee: "Ben", hours: 3, billable: false },
            WorkLog { department: "Support", employee: "Cara", hours: 4, billable: true },
            WorkLog { department: "Design", employee: "Ava", hours: 2, billable: true },
            WorkLog { department: "Support", employee: "Drew", hours: 1, billable: false },
        ];

        let report = build_department_report(&logs);
        let expected = concat!(
            "Design: employees=2, total=10, billable=7, non_billable=3\n",
            "Support: employees=2, total=5, billable=4, non_billable=1\n",
        );

        assert_eq!(report, expected);
    }

    #[test]
    fn report_skips_departments_with_zero_total_hours() {
        let logs = [
            WorkLog { department: "Ops", employee: "Eli", hours: 0, billable: false },
            WorkLog { department: "Ops", employee: "Finn", hours: 0, billable: true },
            WorkLog { department: "QA", employee: "Gia", hours: 2, billable: false },
        ];

        let report = build_department_report(&logs);
        assert_eq!(report, "QA: employees=1, total=2, billable=0, non_billable=2\n");
    }
}
