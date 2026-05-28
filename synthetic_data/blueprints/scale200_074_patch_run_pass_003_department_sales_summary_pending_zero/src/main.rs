use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Order {
    department: &'static str,
    amount: u32,
    paid: bool,
}

fn build_report(orders: &[Order]) -> String {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for order in orders {
        if order.paid {
            *totals.entry(order.department).or_insert(0) += order.amount;
        }
    }

    let mut rows: Vec<(&str, u32)> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::from("Sales Summary\n");
    for (idx, (dept, total)) in rows.iter().enumerate() {
        if idx > 0 {
            out.push('\n');
        }
        out.push_str(&format!("{}: {}", dept, total));
    }
    out
}

fn main() {
    let orders = [
        Order {
            department: "garden",
            amount: 7,
            paid: true,
        },
        Order {
            department: "books",
            amount: 12,
            paid: true,
        },
        Order {
            department: "toys",
            amount: 9,
            paid: false,
        },
        Order {
            department: "garden",
            amount: 5,
            paid: true,
        },
        Order {
            department: "ops",
            amount: 8,
            paid: true,
        },
        Order {
            department: "ops",
            amount: 5,
            paid: true,
        },
        Order {
            department: "books",
            amount: 4,
            paid: false,
        },
    ];

    println!("{}", build_report(&orders));
}
