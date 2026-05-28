struct Order {
    region: &'static str,
    qty: u32,
    unit_price_cents: u32,
    refunded: bool,
}

#[derive(Clone, Copy)]
struct Summary {
    orders: u32,
    revenue_cents: u32,
}

fn main() {
    let orders = vec![
        Order { region: "North", qty: 2, unit_price_cents: 500, refunded: false },
        Order { region: "South", qty: 1, unit_price_cents: 700, refunded: false },
        Order { region: "North", qty: 1, unit_price_cents: 750, refunded: false },
        Order { region: "South", qty: 3, unit_price_cents: 200, refunded: false },
        Order { region: "North", qty: 4, unit_price_cents: 100, refunded: true },
    ];

    let mut north = Summary { orders: 0, revenue_cents: 0 };
    let mut south = Summary { orders: 0, revenue_cents: 0 };

    for order in orders {
        let amount = order.unit_price_cents;
        match order.region {
            "North" => {
                north.orders += 1;
                north.revenue_cents += amount;
            }
            "South" => {
                south.orders += 1;
                south.revenue_cents += amount;
            }
            _ => {}
        }
    }

    print_summary("North", north);
    print_summary("South", south);
    let total_orders = north.orders + south.orders;
    let total_revenue = north.revenue_cents + south.revenue_cents;
    println!("TOTAL: orders={} revenue={}", total_orders, money(total_revenue));
}

fn print_summary(name: &str, summary: Summary) {
    let avg = if summary.orders == 0 {
        0.0
    } else {
        summary.revenue_cents as f64 / summary.orders as f64 / 100.0
    };
    println!(
        "{}: orders={} revenue={} avg={:.2}",
        name,
        summary.orders,
        money(summary.revenue_cents),
        avg
    );
}

fn money(cents: u32) -> String {
    format!("{}.{:02}", cents / 100, cents % 100)
}
