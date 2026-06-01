enum Action {
    Dispatch { warehouse: &'static str },
    Hold { reason: &'static str },
    Reroute { hub: &'static str },
}

struct Shipment {
    id: &'static str,
    action: Action,
}

fn describe_action(action: &Action) -> String {
    match action {
        Action::Dispatch { warehouse } => format!("ship from {}", warehouse),
        Action::Hold { reason } => format!("hold: {}", reason),
        Action::Reroute { hub } => format!("dispatch {}", hub),
    }
}

fn render(shipment: &Shipment) -> String {
    format!("{}: {}", shipment.id, describe_action(&shipment.action))
}

fn main() {
    let shipments = [
        Shipment {
            id: "PKG-17",
            action: Action::Dispatch { warehouse: "warehouse A" },
        },
        Shipment {
            id: "PKG-18",
            action: Action::Hold { reason: "customs" },
        },
        Shipment {
            id: "PKG-19",
            action: Action::Reroute { hub: "hub-2" },
        },
    ];

    for shipment in shipments.iter() {
        println!("{}", render(shipment));
    }
}
