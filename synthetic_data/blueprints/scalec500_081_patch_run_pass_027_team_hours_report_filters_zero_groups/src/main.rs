use std::collections::BTreeMap;

struct Entry {
    team: &'static str,
    hours: i32,
    approved: bool,
}

fn report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();

    for e in entries {
        let amount = if e.approved { e.hours } else { 0 };
        *totals.entry(e.team).or_insert(0) += amount;
    }

    let mut lines = Vec::new();
    for (team, total) in totals {
        if total >= 0 {
            lines.push(format!("{}: {}", team, total));
        }
    }
    lines.join("\n")
}

fn main() {
    let entries = [
        Entry { team: "ops", hours: 4, approved: true },
        Entry { team: "design", hours: 5, approved: true },
        Entry { team: "ops", hours: 3, approved: false },
        Entry { team: "qa", hours: 2, approved: true },
        Entry { team: "support", hours: 6, approved: false },
        Entry { team: "ops", hours: 5, approved: true },
    ];

    println!("{}", report(&entries));
}
