enum Destination {
    Domestic,
    International,
}

enum OrderKind {
    Standard,
    Express,
    Fragile,
    Restricted,
}

struct Order {
    id: &'static str,
    kind: OrderKind,
    destination: Destination,
    locker_pickup: bool,
}

fn label(order: &Order) -> &'static str {
    match (&order.kind, &order.destination, order.locker_pickup) {
        (_, _, true) => "pickup at locker",
        (OrderKind::Express, _, _) => "domestic express",
        (OrderKind::Restricted, Destination::International, _) => "intl customs-hold",
        (OrderKind::Restricted, Destination::Domestic, _) => "domestic restricted",
        (OrderKind::Fragile, Destination::International, _) => "intl fragile",
        (_, Destination::International, _) => "intl standard",
        _ => "domestic standard",
    }
}

fn main() {
    let orders = [
        Order {
            id: "A1",
            kind: OrderKind::Standard,
            destination: Destination::Domestic,
            locker_pickup: true,
        },
        Order {
            id: "B2",
            kind: OrderKind::Express,
            destination: Destination::Domestic,
            locker_pickup: false,
        },
        Order {
            id: "C3",
            kind: OrderKind::Restricted,
            destination: Destination::International,
            locker_pickup: false,
        },
        Order {
            id: "D4",
            kind: OrderKind::Restricted,
            destination: Destination::International,
            locker_pickup: true,
        },
    ];

    for order in orders {
        println!("{} -> {}", order.id, label(&order));
    }
}
