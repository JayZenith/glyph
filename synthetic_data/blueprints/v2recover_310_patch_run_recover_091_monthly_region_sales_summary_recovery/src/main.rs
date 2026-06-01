use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Sale {
    month: &'static str,
    region: &'static str,
    amount: i32,
    closed: bool,
}

fn main() {
    let sales = [
        Sale { month: "2024-01", region: "east", amount: 10, closed: true },
        Sale { month: "2024-01", region: "east", amount: 4, closed: true },
        Sale { month: "2024-01", region: "west", amount: 8, closed: true },
        Sale { month: "2024-02", region: "east", amount: 4, closed: true },
        Sale { month: "2024-02", region: "west", amount: 6, closed: true },
        Sale { month: "2024-02", region: "west", amount: 8, closed: true },
        Sale { month: "2024-01", region: "west", amount: 7, closed: false },
        Sale { month: "2024-02", region: "east", amount: 3, closed: false },
    ];

    let mut totals: BTreeMap<&str, (i32, i32)> = BTreeMap::new();

    for sale in sales {
        let key = sale.month;
        let entry = totals.entry(key).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += sale.amount;
    }

    let mut lines = Vec::new();
    for (month, (count, total)) in totals {
        lines.push(format!("{}: count={} total={}", month, count, total));
    }

    println!("{}", lines.join("\n"));
}
