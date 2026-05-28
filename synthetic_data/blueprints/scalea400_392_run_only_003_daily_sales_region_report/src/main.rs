fn main() {
    let orders = [
        ("North", 120),
        ("South", 80),
        ("North", 35),
        ("West", 40),
        ("South", 70),
    ];

    let regions = ["North", "South", "West"];
    let mut lines = Vec::new();
    let mut total_orders = 0;
    let mut total_revenue = 0;

    for region in regions {
        let mut count = 0;
        let mut revenue = 0;
        for (r, amount) in orders {
            if r == region {
                count += 1;
                revenue += amount;
            }
        }
        total_orders += count;
        total_revenue += revenue;
        lines.push(format!("{}: orders={} revenue={}", region, count, revenue));
    }

    lines.push(format!("TOTAL: orders={} revenue={}", total_orders, total_revenue));
    print!("{}", lines.join("\n"));
}
