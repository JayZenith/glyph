enum ShipmentState {
    Preparing,
    InTransit { delayed: bool },
    Delivered { signed: bool },
    Canceled,
}

struct Shipment {
    code: &'static str,
    state: ShipmentState,
}

fn label(state: &ShipmentState) -> &'static str {
    match state {
        ShipmentState::Preparing => "packed",
        ShipmentState::InTransit { delayed } => {
            if *delayed {
                "delivered"
            } else {
                "hold"
            }
        }
        ShipmentState::Delivered { signed } => {
            if *signed {
                "delivered"
            } else {
                "hold"
            }
        }
        ShipmentState::Canceled => "canceled",
    }
}

fn main() {
    let shipments = [
        Shipment {
            code: "A100",
            state: ShipmentState::Preparing,
        },
        Shipment {
            code: "B205",
            state: ShipmentState::InTransit { delayed: true },
        },
        Shipment {
            code: "C876",
            state: ShipmentState::Delivered { signed: true },
        },
        Shipment {
            code: "D333",
            state: ShipmentState::Canceled,
        },
    ];

    for shipment in shipments {
        println!("{}: {}", shipment.code, label(&shipment.state));
    }
}
