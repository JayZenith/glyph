use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Entry {
    dept: &'static str,
    hours: u32,
    billable: bool,
}

fn main() {
    let entries = [
        Entry { dept: "Engineering", hours: 5, billable: true },
        Entry { dept: "Sales", hours: 4, billable: true },
        Entry { dept: "Engineering", hours: 2, billable: true },
        Entry { dept: "Support", hours: 1, billable: false },
        Entry { dept: "Support", hours: 3, billable: true },
        Entry { dept: "Support", hours: 1, billable: true },
    ];

    let mut by_dept: BTreeMap<&str, (u32, u32)> = BTreeMap::new();
    for e in entries {
        let row = by_dept.entry(e.dept).or_insert((0, 0));
        row.0 += e.hours;
        row.1 += 1;
    }

    let mut rows: Vec<_> = by_dept.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut total = 0;
    for (dept, (hours, count)) in &rows {
        total += *hours;
        println!("{}: {}h ({} entries)", dept, hours, count);
    }
    println!("Total billable: {}h", total);
}
