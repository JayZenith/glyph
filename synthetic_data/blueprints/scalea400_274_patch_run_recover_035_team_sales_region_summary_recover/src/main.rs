use std::collections::BTreeMap;

struct Sale {
    region: &'static str,
    amount: u32,
    active: bool,
}

fn main() {
    let sales = vec![
        Sale { region: "North", amount: 90, active: true },
        Sale { region: "South", amount: 80, active: false },
        Sale { region: "East", amount: 40, active: true },
        Sale { region: "West", amount: 120, active: true },
        Sale { region: "North", amount: 60, active: true },
        Sale { region: "East", amount: 60, active: true },
        Sale { region: "West", amount: 30, active: true },
        Sale { region: "South", amount: 50, active: true },
    ];

    let mut totals: BTreeMap<&str, (u32, u32)> = BTreeMap::new();
    for sale in sales {
        let entry = totals.entry(sale.region).or_insert((0, 0));
        entry.0 += sale.amount;
        entry.1 += 1;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::new();
    for (idx, (region, (total, count))) in rows.iter().enumerate() {
        if *total >= 100 {
            if idx > 0 {
                out.push('\n');
            }
            out.push_str(&format!("{} | orders={} | total={}", region, total, count));
        }
    }

    print!("{}", out);
}
