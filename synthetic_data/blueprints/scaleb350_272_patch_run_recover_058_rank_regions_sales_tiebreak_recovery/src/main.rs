use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Sale {
    region: &'static str,
    amount: u32,
}

fn main() {
    let sales = [
        Sale { region: "North", amount: 120 },
        Sale { region: "South", amount: 90 },
        Sale { region: "East", amount: 100 },
        Sale { region: "West", amount: 80 },
        Sale { region: "South", amount: 60 },
        Sale { region: "North", amount: 80 },
        Sale { region: "East", amount: 100 },
        Sale { region: "West", amount: 100 },
        Sale { region: "Central", amount: 70 },
        Sale { region: "Central", amount: 50 },
        Sale { region: "South", amount: 50 },
    ];

    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();
    for sale in sales {
        let entry = totals.entry(sale.region).or_insert((0, 0));
        entry.0 += sale.amount;
        entry.1 += 1;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| {
        b.1 .0
            .cmp(&a.1 .0)
            .then(a.0.cmp(b.0))
    });

    let mut out = Vec::new();
    for (idx, (region, (total, orders))) in rows.iter().enumerate() {
        out.push(format!("{}. {} - total={} orders={}", idx + 1, region, total, orders));
    }

    println!("{}", out.join("\n"));
}
