enum ItemKind {
    Standard,
    Perishable,
    Digital,
    Oversized,
}

struct Order {
    name: &'static str,
    kind: ItemKind,
    fragile: bool,
}

fn action(order: &Order) -> &'static str {
    match (&order.kind, order.fragile) {
        (ItemKind::Digital, _) => "email-code",
        (ItemKind::Perishable, true) => "double-box",
        (ItemKind::Perishable, false) => "pack-cold",
        (ItemKind::Oversized, _) => "queue-standard",
        (ItemKind::Standard, true) => "add-padding",
        (ItemKind::Standard, false) => "queue-standard",
    }
}

fn main() {
    let orders = [
        Order {
            name: "small-box",
            kind: ItemKind::Standard,
            fragile: false,
        },
        Order {
            name: "frozen-meal",
            kind: ItemKind::Perishable,
            fragile: false,
        },
        Order {
            name: "gift-card",
            kind: ItemKind::Digital,
            fragile: false,
        },
        Order {
            name: "wardrobe",
            kind: ItemKind::Oversized,
            fragile: true,
        },
    ];

    for order in orders {
        println!("{}: {}", order.name, action(&order));
    }
}
