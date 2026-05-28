use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Product {
    name: &'static str,
    sold: u32,
    revenue_cents: u32,
}

fn main() {
    let mut items = vec![
        Product { name: "Berry", sold: 7, revenue_cents: 1540 },
        Product { name: "Apple", sold: 5, revenue_cents: 1250 },
        Product { name: "Banana", sold: 7, revenue_cents: 1400 },
        Product { name: "Apricot", sold: 7, revenue_cents: 1540 },
    ];

    items.sort_by(|a, b| {
        b.sold.cmp(&a.sold)
            .then_with(|| a.revenue_cents.cmp(&b.revenue_cents))
            .then_with(|| b.name.cmp(a.name))
    });

    for (idx, item) in items.iter().enumerate() {
        println!(
            "{}. {} | sold={} | revenue={:.2}",
            idx + 1,
            item.name,
            item.sold,
            item.revenue_cents as f64 / 100.0
        );
    }
}
