use std::collections::BTreeMap;

#[derive(Default)]
struct Totals {
    orders: u32,
    units: u32,
    revenue: u32,
}

fn main() {
    let input = [
        "North,shipped,3,30",
        "South,shipped,2,50",
        "North,pending,4,40",
        "West,shipped,5,20",
        "South,cancelled,1,10",
        "North,shipped,5,50",
        "South,shipped,4,40",
    ];

    let mut by_region: BTreeMap<&str, Totals> = BTreeMap::new();

    for line in input {
        let parts: Vec<&str> = line.split(',').collect();
        let region = parts[0];
        let status = parts[1];
        let units: u32 = parts[2].parse().unwrap();
        let revenue: u32 = parts[3].parse().unwrap();

        if status == "cancelled" {
            continue;
        }

        let entry = by_region.entry(region).or_default();
        entry.orders += 1;
        entry.units += 1;
        entry.revenue += revenue;
    }

    println!("Summary Report");
    let mut grand_total = 0;
    for (region, totals) in by_region {
        grand_total += totals.units;
        println!(
            "{}: orders={} units={} revenue={}",
            region, totals.orders, totals.units, totals.revenue
        );
    }
    println!("Grand total revenue={}", grand_total);
}
