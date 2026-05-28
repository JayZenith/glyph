use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub project: &'static str,
    pub hours: u32,
    pub billable: bool,
}

pub fn summarize(entries: &[Entry]) -> Vec<String> {
    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for entry in entries {
        let slot = totals.entry(entry.project).or_insert((0, 0));
        if entry.billable {
            slot.0 += entry.hours;
        } else {
            slot.1 += entry.hours;
        }
    }

    let mut rows: Vec<_> = totals
        .into_iter()
        .map(|(project, (billable, non_billable))| {
            let total = billable + non_billable;
            (project, billable, non_billable, total)
        })
        .collect();

    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(project, billable, non_billable, total)| {
            format!(
                "{project}: total={total}h billable={billable}h non_billable={non_billable}h"
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{summarize, Entry};

    #[test]
    fn groups_hours_and_orders_by_total_desc_then_name() {
        let entries = [
            Entry {
                project: "beta",
                hours: 2,
                billable: true,
            },
            Entry {
                project: "alpha",
                hours: 1,
                billable: false,
            },
            Entry {
                project: "alpha",
                hours: 4,
                billable: true,
            },
            Entry {
                project: "beta",
                hours: 3,
                billable: false,
            },
            Entry {
                project: "gamma",
                hours: 5,
                billable: true,
            },
        ];

        let report = summarize(&entries);

        assert_eq!(
            report,
            vec![
                "alpha: total=5h billable=4h non_billable=1h",
                "beta: total=5h billable=2h non_billable=3h",
                "gamma: total=5h billable=5h non_billable=0h",
            ]
        );
    }

    #[test]
    fn skips_projects_with_zero_total_hours() {
        let entries = [
            Entry {
                project: "idle",
                hours: 0,
                billable: true,
            },
            Entry {
                project: "active",
                hours: 2,
                billable: false,
            },
            Entry {
                project: "idle",
                hours: 0,
                billable: false,
            },
        ];

        let report = summarize(&entries);

        assert_eq!(report, vec!["active: total=2h billable=0h non_billable=2h"]);
    }
}
