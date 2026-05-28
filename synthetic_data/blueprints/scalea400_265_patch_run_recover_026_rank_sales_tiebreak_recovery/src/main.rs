#[derive(Clone, Debug)]
struct Seller {
    name: &'static str,
    sales: u32,
    refunds: u32,
}

fn main() {
    let mut sellers = vec![
        Seller { name: "Bob", sales: 17, refunds: 2 },
        Seller { name: "Ada", sales: 17, refunds: 1 },
        Seller { name: "Dan", sales: 12, refunds: 3 },
        Seller { name: "Cara", sales: 17, refunds: 1 },
        Seller { name: "Eve", sales: 12, refunds: 0 },
    ];

    sellers.sort_by(|a, b| {
        b.sales
            .cmp(&a.sales)
            .then(a.name.cmp(&b.name))
            .then(a.refunds.cmp(&b.refunds))
    });

    let mut lines = Vec::new();
    for (i, s) in sellers.iter().enumerate() {
        lines.push(format!(
            "{}. {} | sales={} | refunds={}",
            i + 1,
            s.name,
            s.sales,
            s.refunds
        ));
    }

    println!("{}", lines.join("\n"));
}
