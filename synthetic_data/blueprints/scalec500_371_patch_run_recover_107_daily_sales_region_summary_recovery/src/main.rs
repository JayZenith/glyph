use std::collections::BTreeMap;

#[derive(Default)]
struct Stats {
    orders: u32,
    units: i32,
    net: i32,
}

fn main() {
    let input = [
        "North,Widget,2,10",
        "East,Gadget,3,15",
        "North,Widget,1,-3",
        "West,Gizmo,4,12",
        "East,Broken,2,0",
        "West,Gizmo,1,5",
        "South,Widget,1,-2",
        "North,Bolt,2,15",
        "BadRow",
        "West,Gizmo,oops,9",
    ];

    let mut by_region: BTreeMap<&str, Stats> = BTreeMap::new();

    for line in input {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 4 {
            continue;
        }

        let region = parts[0];
        let units: i32 = parts[2].parse().unwrap_or(0);
        let amount: i32 = parts[3].parse().unwrap_or(0);

        let stats = by_region.entry(region).or_default();
        stats.orders += 1;
        stats.units += units;
        stats.net += amount;
    }

    let mut rows: Vec<_> = by_region.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    for (region, stats) in rows {
        println!(
            "{} | orders={} | units={} | net={}",
            region, stats.orders, stats.units, stats.net
        );
    }
}
