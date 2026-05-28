struct Order {
    region: &'static str,
    qty: u32,
    shipped: bool,
}

fn main() {
    let orders = vec![
        Order { region: "North", qty: 4, shipped: true },
        Order { region: "South", qty: 3, shipped: false },
        Order { region: "North", qty: 1, shipped: false },
        Order { region: "South", qty: 6, shipped: true },
        Order { region: "North", qty: 3, shipped: true },
    ];

    let mut north = 0;
    let mut south = 0;
    let mut total = 0;

    for order in orders {
        match order.region {
            "North" => north += order.qty,
            "South" => south += order.qty,
            _ => {}
        }
        if order.shipped {
            total += order.qty;
        }
    }

    println!("North: {}", north);
    println!("South: {}", south);
    println!("TOTAL: {}", total);
}
