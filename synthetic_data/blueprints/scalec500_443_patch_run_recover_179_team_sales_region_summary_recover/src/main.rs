use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy)]
struct Sale {
    region: &'static str,
    rep: &'static str,
    amount: u32,
    closed: bool,
}

fn main() {
    let sales = vec![
        Sale { region: "North", rep: "Ana", amount: 120, closed: true },
        Sale { region: "South", rep: "Bo", amount: 90, closed: false },
        Sale { region: "North", rep: "Cy", amount: 60, closed: true },
        Sale { region: "West", rep: "Ana", amount: 80, closed: true },
        Sale { region: "South", rep: "Dee", amount: 120, closed: true },
        Sale { region: "West", rep: "Eli", amount: 100, closed: true },
    ];

    let mut per_region: BTreeMap<&str, (u32, u32, BTreeSet<&str>)> = BTreeMap::new();
    let mut total_orders = 0_u32;
    let mut total_revenue = 0_u32;

    for sale in &sales {
        let entry = per_region.entry(sale.region).or_insert((0, 0, BTreeSet::new()));
        entry.0 += 1;
        entry.1 += sale.amount;
        entry.2.insert(sale.rep);
        total_orders += 1;
        total_revenue += sale.amount;
    }

    let mut rows: Vec<_> = per_region
        .into_iter()
        .map(|(region, (orders, revenue, reps))| (region, orders, revenue, reps.len()))
        .collect();

    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::new();
    for (i, (region, orders, revenue, rep_count)) in rows.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str(&format!(
            "{} | orders={} | reps={} | revenue={}",
            region, orders, rep_count, revenue
        ));
    }
    out.push_str(&format!(
        "\nOVERALL | orders={} | revenue={}",
        total_orders, total_revenue
    ));

    print!("{}", out);
}
