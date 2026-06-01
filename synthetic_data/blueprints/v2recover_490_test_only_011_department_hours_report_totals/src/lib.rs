use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub department: &'static str,
    pub hours: u32,
    pub billable: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DepartmentSummary {
    pub total_hours: u32,
    pub billable_hours: u32,
    pub entry_count: usize,
}

pub fn summarize(entries: &[Entry]) -> BTreeMap<&'static str, DepartmentSummary> {
    let mut out = BTreeMap::new();

    for entry in entries {
        let summary = out.entry(entry.department).or_insert(DepartmentSummary {
            total_hours: 0,
            billable_hours: 0,
            entry_count: 0,
        });

        summary.total_hours += entry.hours;
        if entry.billable {
            summary.billable_hours += entry.hours;
        }
        summary.entry_count += 1;
    }

    out
}

pub fn report_lines(entries: &[Entry]) -> Vec<String> {
    summarize(entries)
        .into_iter()
        .map(|(dept, summary)| {
            format!(
                "{}: total={} billable={} entries={}",
                dept, summary.total_hours, summary.billable_hours, summary.entry_count
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entries() -> Vec<Entry> {
        vec![
            Entry {
                department: "Design",
                hours: 5,
                billable: true,
            },
            Entry {
                department: "Ops",
                hours: 3,
                billable: false,
            },
            Entry {
                department: "Design",
                hours: 2,
                billable: false,
            },
            Entry {
                department: "Ops",
                hours: 4,
                billable: true,
            },
            Entry {
                department: "Sales",
                hours: 6,
                billable: true,
            },
        ]
    }

    #[test]
    fn summarizes_each_department() {
        let summary = summarize(&sample_entries());

        assert_eq!(
            summary.get("Design"),
            Some(&DepartmentSummary {
                total_hours: 7,
                billable_hours: 5,
                entry_count: 2,
            })
        );
        assert_eq!(
            summary.get("Ops"),
            Some(&DepartmentSummary {
                total_hours: 7,
                billable_hours: 4,
                entry_count: 2,
            })
        );
        assert_eq!(
            summary.get("Sales"),
            Some(&DepartmentSummary {
                total_hours: 6,
                billable_hours: 6,
                entry_count: 1,
            })
        );
    }

    #[test]
    fn report_lines_are_sorted_by_department_name() {
        let lines = report_lines(&sample_entries());
        assert_eq!(
            lines,
            vec![
                "Design: total=7 billable=5 entries=2",
                "Ops: total=7 billable=4 entries=2",
                "Sales: total=6 billable=6 entries=1",
            ]
        );
    }

    #[test]
    fn empty_input_produces_no_lines() {
        let lines = report_lines(&[]);
        assert!(lines.is_empty());
    }
}
