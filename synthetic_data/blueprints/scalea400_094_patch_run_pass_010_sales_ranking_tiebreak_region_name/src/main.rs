use std::cmp::Reverse;

#[derive(Debug)]
struct Store {
    name: &'static str,
    region: &'static str,
    sales: u32,
    orders: u32,
}

fn main() {
    let mut stores = vec![
        Store { name: "Alpha", region: "North", sales: 120, orders: 7 },
        Store { name: "Gamma", region: "South", sales: 120, orders: 5 },
        Store { name: "Delta", region: "East", sales: 120, orders: 7 },
        Store { name: "Beta", region: "West", sales: 95, orders: 4 },
        Store { name: "Epsilon", region: "East", sales: 95, orders: 9 },
    ];

    stores.sort_by_key(|s| (Reverse(s.sales), s.region, s.name, Reverse(s.orders)));

    let out = stores
        .iter()
        .take(4)
        .enumerate()
        .map(|(i, s)| {
            format!(
                "{}. {} / {} => sales={}, orders={}",
                i + 1,
                s.region,
                s.name,
                s.sales,
                s.orders
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    print!("{}", out);
}
