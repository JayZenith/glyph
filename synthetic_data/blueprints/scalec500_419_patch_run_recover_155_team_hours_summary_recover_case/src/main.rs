use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Entry {
    team: &'static str,
    hours: i32,
    active: bool,
}

fn main() {
    let entries = [
        Entry { team: "eng", hours: 5, active: true },
        Entry { team: "ops", hours: 4, active: true },
        Entry { team: "eng", hours: 2, active: false },
        Entry { team: "sales", hours: 0, active: true },
        Entry { team: "ops", hours: 5, active: true },
        Entry { team: "eng", hours: 4, active: true },
    ];

    let mut groups: BTreeMap<&str, (i32, i32)> = BTreeMap::new();
    let mut grand_total = 0;
    let mut active_count = 0;

    for e in entries {
        let slot = groups.entry(e.team).or_insert((0, 0));
        if e.active {
            slot.0 += 1;
            slot.1 += e.hours;
            grand_total += e.hours;
            active_count += 1;
        }
    }

    let mut out = String::from("TEAM HOURS\n");
    for (team, (count, total)) in groups {
        let avg = if count == 0 { 0.0 } else { total as f64 / count as f64 };
        out.push_str(&format!("{}: count={} total={} avg={:.1}\n", team, count, total, avg));
    }
    out.push_str(&format!("GRAND total={} active={}", grand_total, entries.len()));
    print!("{}", out);
}
