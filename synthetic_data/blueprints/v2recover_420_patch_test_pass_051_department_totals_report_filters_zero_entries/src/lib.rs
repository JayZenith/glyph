use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Entry {
    pub department: &'static str,
    pub hours: u32,
    pub active: bool,
}

pub fn department_report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for entry in entries {
        *totals.entry(entry.department).or_insert(0) += entry.hours;
    }

    let mut lines = Vec::new();
    for (dept, total) in totals {
        lines.push(format!("{}:{}", dept, total));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{department_report, Entry};

    #[test]
    fn aggregates_only_active_and_skips_zero_totals() {
        let entries = [
            Entry { department: "engineering", hours: 5, active: true },
            Entry { department: "support", hours: 0, active: true },
            Entry { department: "engineering", hours: 2, active: false },
            Entry { department: "sales", hours: 3, active: true },
            Entry { department: "support", hours: 4, active: false },
        ];

        assert_eq!(department_report(&entries), "engineering:5\nsales:3");
    }

    #[test]
    fn sorts_by_total_desc_then_department_name() {
        let entries = [
            Entry { department: "beta", hours: 4, active: true },
            Entry { department: "alpha", hours: 4, active: true },
            Entry { department: "ops", hours: 2, active: true },
            Entry { department: "ops", hours: 1, active: true },
        ];

        assert_eq!(department_report(&entries), "alpha:4\nbeta:4\nops:3");
    }

    #[test]
    fn empty_when_no_active_hours_remain() {
        let entries = [
            Entry { department: "finance", hours: 0, active: true },
            Entry { department: "finance", hours: 7, active: false },
        ];

        assert_eq!(department_report(&entries), "");
    }
}
