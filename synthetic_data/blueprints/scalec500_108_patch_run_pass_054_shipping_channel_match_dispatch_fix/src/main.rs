enum Fulfillment {
    Ship { express: bool },
    Pickup { curbside: bool },
    Digital { license_email: bool },
}

struct Order {
    code: &'static str,
    fulfillment: Fulfillment,
}

fn channel_label(f: &Fulfillment) -> &'static str {
    match f {
        Fulfillment::Ship { express: true } => "expedited",
        Fulfillment::Ship { express: false } => "standard",
        Fulfillment::Pickup { .. } => "front desk pickup",
        Fulfillment::Digital { .. } => "download",
    }
}

fn main() {
    let orders = [
        Order {
            code: "A100",
            fulfillment: Fulfillment::Ship { express: true },
        },
        Order {
            code: "B205",
            fulfillment: Fulfillment::Pickup { curbside: true },
        },
        Order {
            code: "C310",
            fulfillment: Fulfillment::Digital {
                license_email: true,
            },
        },
        Order {
            code: "D440",
            fulfillment: Fulfillment::Ship { express: false },
        },
    ];

    for order in orders {
        println!("{}: {}", order.code, channel_label(&order.fulfillment));
    }
}
