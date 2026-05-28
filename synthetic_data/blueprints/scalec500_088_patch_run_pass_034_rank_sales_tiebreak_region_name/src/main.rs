#[derive(Clone, Copy)]
struct Store {
    name: &'static str,
    region: &'static str,
    sales: u32,
}

fn main() {
    let mut stores = vec![
        Store { name: "Beta", region: "South", sales: 18 },
        Store { name: "Gamma", region: "West", sales: 12 },
        Store { name: "Alpha", region: "North", sales: 18 },
        Store { name: "Delta", region: "East", sales: 18 },
        Store { name: "Epsilon", region: "North", sales: 12 },
    ];

    stores.sort_by(|a, b| {
        b.sales
            .cmp(&a.sales)
            .then_with(|| a.name.cmp(b.name))
            .then_with(|| a.region.cmp(b.region))
    });

    for (idx, store) in stores.iter().enumerate() {
        println!(
            "{}. {} ({}) - {}",
            idx + 1,
            store.name,
            store.region,
            store.sales
        );
    }
}
