use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Entry {
    dept: &'static str,
    person: &'static str,
    hours: u32,
    active: bool,
}

fn main() {
    let entries = [
        Entry { dept: "Sales", person: "Ava", hours: 5, active: true },
        Entry { dept: "Engineering", person: "Ben", hours: 8, active: true },
        Entry { dept: "Sales", person: "Cara", hours: 0, active: true },
        Entry { dept: "Support", person: "Dan", hours: 3, active: true },
        Entry { dept: "Engineering", person: "Eli", hours: 4, active: true },
        Entry { dept: "Support", person: "Fay", hours: 5, active: true },
        Entry { dept: "Sales", person: "Gus", hours: 6, active: true },
        Entry { dept: "Engineering", person: "Hal", hours: 2, active: false },
    ];

    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();

    for e in entries {
        let entry = totals.entry(e.dept).or_insert((0, 0));
        entry.0 += e.hours;
        entry.1 += 1;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    for (dept, (hours, people)) in rows {
        println!("{}: {}h ({} people)", dept, hours, people);
    }
}
