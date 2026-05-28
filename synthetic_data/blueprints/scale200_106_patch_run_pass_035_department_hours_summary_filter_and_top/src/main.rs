use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy)]
struct Entry {
    employee: &'static str,
    department: &'static str,
    hours: i32,
    billable: bool,
}

fn main() {
    let entries = [
        Entry { employee: "Ava", department: "Engineering", hours: 3, billable: true },
        Entry { employee: "Ben", department: "Engineering", hours: 4, billable: true },
        Entry { employee: "Ava", department: "Engineering", hours: 2, billable: false },
        Entry { employee: "Cara", department: "Sales", hours: 5, billable: true },
        Entry { employee: "Cara", department: "Sales", hours: 3, billable: true },
        Entry { employee: "Drew", department: "Sales", hours: 0, billable: true },
        Entry { employee: "Eli", department: "Support", hours: 6, billable: false },
    ];

    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();
    let mut people: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();

    for entry in entries {
        *totals.entry(entry.department).or_insert(0) += entry.hours;
        people
            .entry(entry.department)
            .or_default()
            .insert(entry.employee);
    }

    for (dept, total) in &totals {
        let count = people.get(dept).map(|s| s.len()).unwrap_or(0);
        println!("{}: total={}, people={}", dept, total, count);
    }

    let top = totals
        .iter()
        .max_by_key(|(_, total)| *total)
        .map(|(dept, _)| *dept)
        .unwrap_or("none");

    println!("Top department: {}", top);
}
