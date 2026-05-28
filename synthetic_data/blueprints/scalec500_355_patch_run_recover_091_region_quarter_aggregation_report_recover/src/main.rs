use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Sale {
    region: &'static str,
    quarter: &'static str,
    amount: i32,
    cancelled: bool,
}

fn main() {
    let sales = vec![
        Sale { region: "North", quarter: "Q1", amount: 120, cancelled: false },
        Sale { region: "North", quarter: "Q2", amount: 80, cancelled: false },
        Sale { region: "South", quarter: "Q2", amount: 70, cancelled: false },
        Sale { region: "South", quarter: "Q3", amount: 50, cancelled: false },
        Sale { region: "West", quarter: "Q4", amount: 60, cancelled: false },
        Sale { region: "West", quarter: "Q1", amount: 40, cancelled: false },
        Sale { region: "North", quarter: "Q3", amount: 30, cancelled: true },
        Sale { region: "South", quarter: "Q4", amount: 10, cancelled: true },
    ];

    let mut by_region: BTreeMap<&str, BTreeMap<&str, i32>> = BTreeMap::new();
    for sale in sales {
        let quarters = by_region.entry(sale.region).or_default();
        *quarters.entry(sale.quarter).or_insert(0) += sale.amount;
    }

    let mut lines = Vec::new();
    for (region, quarters) in by_region {
        let mut total = 0;
        let mut parts = Vec::new();
        for q in ["Q4", "Q3", "Q2", "Q1"] {
            if let Some(amount) = quarters.get(q) {
                total += amount;
                parts.push(format!("{}={}", q, amount));
            }
        }
        parts.push(format!("Total={}", total));
        lines.push(format!("{}: {}", region, parts.join(" ")));
    }

    println!("{}", lines.join("\n"));
}
