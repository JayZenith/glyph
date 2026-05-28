use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Sale {
    region: &'static str,
    units: u32,
    price: u32,
    refunded: bool,
}

fn main() {
    let sales = [
        Sale { region: "North", units: 2, price: 10, refunded: false },
        Sale { region: "South", units: 5, price: 7, refunded: true },
        Sale { region: "East", units: 1, price: 20, refunded: false },
        Sale { region: "North", units: 3, price: 5, refunded: false },
        Sale { region: "West", units: 4, price: 8, refunded: false },
        Sale { region: "East", units: 5, price: 4, refunded: false },
        Sale { region: "West", units: 2, price: 4, refunded: false },
        Sale { region: "North", units: 1, price: 10, refunded: false },
    ];

    let mut totals: BTreeMap<&str, (u32, u32, u32)> = BTreeMap::new();
    for s in sales {
        if s.refunded {
            continue;
        }
        let entry = totals.entry(s.region).or_insert((0, 0, 0));
        entry.0 += 1;
        entry.1 += 1;
        entry.2 += s.price;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let out = rows
        .into_iter()
        .filter(|(_, (_, units, _))| *units >= 5)
        .map(|(region, (orders, units, revenue))| {
            format!("{region}: orders={orders} units={units} revenue={revenue}")
        })
        .collect::<Vec<_>>()
        .join("\n");

    print!("{out}");
}
