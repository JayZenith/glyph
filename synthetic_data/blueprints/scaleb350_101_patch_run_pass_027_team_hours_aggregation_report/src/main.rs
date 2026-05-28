use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Entry {
    team: Option<&'static str>,
    hours: u32,
    billable: bool,
}

fn main() {
    let entries = [
        Entry { team: Some("ops"), hours: 5, billable: true },
        Entry { team: Some("design"), hours: 3, billable: true },
        Entry { team: None, hours: 2, billable: true },
        Entry { team: Some("ops"), hours: 7, billable: true },
        Entry { team: Some("design"), hours: 4, billable: true },
        Entry { team: None, hours: 5, billable: true },
        Entry { team: Some("ops"), hours: 9, billable: false },
    ];

    let mut grouped: BTreeMap<String, (u32, u32)> = BTreeMap::new();
    let mut grand_total = 0;

    for entry in entries {
        if !entry.billable {
            continue;
        }

        let key = entry.team.unwrap_or("UNASSIGNED").to_string();
        let stats = grouped.entry(key).or_insert((0, 0));
        stats.0 += 1;
        stats.1 += entry.hours;
        grand_total += entry.hours;
    }

    for (team, (hours, count)) in grouped {
        println!("{}: {}h ({} entries)", team, hours, count);
    }
    println!("GRAND TOTAL: {}h", grand_total);
}
