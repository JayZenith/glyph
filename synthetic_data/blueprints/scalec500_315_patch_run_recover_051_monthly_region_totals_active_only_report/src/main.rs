use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Event {
    month: &'static str,
    region: &'static str,
    active: bool,
    users: u32,
}

fn main() {
    let events = [
        Event { month: "2024-01", region: "east", active: true, users: 2 },
        Event { month: "2024-01", region: "east", active: false, users: 4 },
        Event { month: "2024-01", region: "west", active: true, users: 1 },
        Event { month: "2024-02", region: "west", active: true, users: 1 },
        Event { month: "2024-02", region: "north", active: true, users: 1 },
        Event { month: "2024-02", region: "east", active: true, users: 1 },
        Event { month: "2024-02", region: "south", active: false, users: 5 },
    ];

    let mut months: BTreeMap<&str, BTreeMap<&str, u32>> = BTreeMap::new();
    let mut grand_total = 0u32;

    for e in events {
        let month_entry = months.entry(e.month).or_default();
        *month_entry.entry(e.region).or_default() += e.users;
        grand_total += e.users;
    }

    let mut lines = Vec::new();
    for (month, regions) in months {
        lines.push(month.to_string());
        let mut subtotal = 0u32;
        for (region, count) in regions {
            lines.push(format!("  {}: {}", region, count));
            subtotal += count;
        }
        lines.push(format!("  subtotal: {}", subtotal));
    }
    lines.push(format!("GRAND TOTAL: {}", grand_total));

    print!("{}", lines.join("\n"));
}
