use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy)]
struct Sale {
    rep: &'static str,
    region: &'static str,
    units: u32,
    returned: bool,
}

fn main() {
    let sales = [
        Sale { rep: "Ava", region: "North", units: 8, returned: false },
        Sale { rep: "Ben", region: "East", units: 5, returned: false },
        Sale { rep: "Ava", region: "North", units: 3, returned: true },
        Sale { rep: "Cara", region: "East", units: 8, returned: false },
        Sale { rep: "Dan", region: "West", units: 4, returned: false },
        Sale { rep: "Eli", region: "West", units: 7, returned: false },
        Sale { rep: "Fay", region: "East", units: 2, returned: true },
    ];

    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();
    let mut qualified: BTreeSet<&str> = BTreeSet::new();

    for sale in sales {
        *totals.entry(sale.region).or_insert(0) += sale.units;
        if sale.units >= 5 {
            qualified.insert(sale.rep);
        }
    }

    let mut top_region = "";
    let mut top_units = 0;
    for (region, units) in &totals {
        if *units >= top_units {
            top_units = *units;
            top_region = region;
        }
    }

    println!("Region totals:");
    for (region, units) in &totals {
        println!("{} => {}", region, units);
    }
    println!("Qualified reps: {}", qualified.len());
    println!("Top region: {} ({})", top_region, top_units);
}
