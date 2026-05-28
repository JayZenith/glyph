use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Entry {
    dept: &'static str,
    hours: u32,
    active: bool,
}

fn build_report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();
    for e in entries {
        *totals.entry(e.dept).or_insert(0) += e.hours;
    }

    let active_departments = totals.len();
    let total_hours: u32 = totals.values().sum();
    let top = totals
        .iter()
        .max_by_key(|(_, hours)| *hours)
        .map(|(dept, hours)| format!("{} ({})", dept, hours))
        .unwrap_or_else(|| "none".to_string());

    let mut lines = vec![
        format!("ACTIVE DEPARTMENTS: {}", active_departments),
        format!("TOTAL HOURS: {}", total_hours),
        format!("TOP: {}", top),
        "DETAILS:".to_string(),
    ];

    for (dept, hours) in totals {
        lines.push(format!("- {}: {}", dept, hours));
    }

    lines.join("\n")
}

fn main() {
    let entries = [
        Entry {
            dept: "Ops",
            hours: 5,
            active: true,
        },
        Entry {
            dept: "Design",
            hours: 6,
            active: true,
        },
        Entry {
            dept: "Ops",
            hours: 3,
            active: true,
        },
        Entry {
            dept: "QA",
            hours: 0,
            active: false,
        },
        Entry {
            dept: "QA",
            hours: 3,
            active: true,
        },
        Entry {
            dept: "Support",
            hours: 4,
            active: false,
        },
    ];

    println!("{}", build_report(&entries));
}
