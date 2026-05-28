use std::collections::BTreeMap;

struct Entry {
    team: &'static str,
    hours: f32,
    billable: bool,
}

fn main() {
    let entries = vec![
        Entry { team: "ENG", hours: 2.5, billable: true },
        Entry { team: "ENG", hours: 3.5, billable: false },
        Entry { team: "OPS", hours: 4.0, billable: true },
        Entry { team: "SALES", hours: 5.0, billable: true },
        Entry { team: "OPS", hours: 3.0, billable: true },
        Entry { team: "ENG", hours: 3.0, billable: true },
        Entry { team: "SALES", hours: 3.5, billable: false },
    ];

    let mut groups: BTreeMap<&str, (usize, f32)> = BTreeMap::new();
    let mut grand_total = 0.0;
    let mut grand_count = 0usize;

    for e in &entries {
        if e.billable {
            let entry = groups.entry(e.team).or_insert((0, 0.0));
            entry.0 += 1;
            entry.1 += e.hours;
            grand_total += e.hours;
            grand_count += 1;
        }
    }

    let mut lines = Vec::new();
    for (team, (count, total)) in groups {
        let avg = if count == 0 { 0.0 } else { total / count as f32 };
        lines.push(format!("{}: count={} total={:.1} avg={:.1}", team, count, total, avg));
    }
    lines.push(format!("ALL: count={} total={:.1}", grand_count, grand_total));

    println!("{}", lines.join("\n"));
}
