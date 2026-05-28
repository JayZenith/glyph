use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Entry {
    pub department: &'static str,
    pub employee: &'static str,
    pub hours: u32,
    pub active: bool,
}

pub fn department_report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, (u32, usize)> = BTreeMap::new();

    for entry in entries {
        let slot = totals.entry(entry.department).or_insert((0, 0));
        slot.0 += entry.hours;
        slot.1 += 1;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(dept, (hours, people))| format!("{}: {}h ({})", dept, hours, people))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_inactive_and_sorts_by_hours_desc_then_name() {
        let entries = vec![
            Entry { department: "Ops", employee: "A", hours: 5, active: true },
            Entry { department: "Sales", employee: "B", hours: 8, active: true },
            Entry { department: "Ops", employee: "C", hours: 4, active: false },
            Entry { department: "HR", employee: "D", hours: 8, active: true },
            Entry { department: "Sales", employee: "E", hours: 1, active: false },
        ];

        assert_eq!(
            department_report(&entries),
            "HR: 8h [1 active]\nSales: 8h [1 active]\nOps: 5h [1 active]"
        );
    }

    #[test]
    fn combines_multiple_active_entries_per_department() {
        let entries = vec![
            Entry { department: "Engineering", employee: "A", hours: 3, active: true },
            Entry { department: "Engineering", employee: "B", hours: 7, active: true },
            Entry { department: "Support", employee: "C", hours: 6, active: true },
        ];

        assert_eq!(
            department_report(&entries),
            "Engineering: 10h [2 active]\nSupport: 6h [1 active]"
        );
    }

    #[test]
    fn empty_when_no_active_entries() {
        let entries = vec![
            Entry { department: "Finance", employee: "A", hours: 4, active: false },
            Entry { department: "Ops", employee: "B", hours: 2, active: false },
        ];

        assert_eq!(department_report(&entries), "");
    }
}
