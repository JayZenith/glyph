fn main() {
    let orders = [
        ("North", 12),
        ("South", 5),
        ("North", 5),
        ("West", 9),
        ("South", 7),
    ];

    let regions = ["North", "South", "West"];

    println!("Sales summary");

    let mut total_units = 0;
    for region in regions {
        let mut orders_count = 0;
        let mut units = 0;
        for (name, qty) in orders {
            if name == region {
                orders_count += 1;
                units += qty;
                total_units += qty;
            }
        }
        println!("{}: {} orders, {} units", region, orders_count, units);
    }

    println!("Total units: {}", total_units);
}
