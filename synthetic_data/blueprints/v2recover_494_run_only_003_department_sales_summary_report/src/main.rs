fn main() {
    let orders = [
        ("North", 120),
        ("South", 80),
        ("North", 85),
        ("West", 90),
        ("South", 20),
    ];

    let regions = ["North", "South", "West"];
    let mut lines = Vec::new();
    let mut grand_total = 0;

    for region in regions {
        let mut count = 0;
        let mut total = 0;
        for (order_region, amount) in orders {
            if order_region == region {
                count += 1;
                total += amount;
            }
        }
        grand_total += total;
        lines.push(format!("{}: {} orders, total {}", region, count, total));
    }

    lines.push(format!("Grand total: {}", grand_total));
    println!("{}", lines.join("\n"));
}
