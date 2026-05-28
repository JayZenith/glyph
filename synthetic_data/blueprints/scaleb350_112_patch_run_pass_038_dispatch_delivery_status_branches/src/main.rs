enum Shipment {
    Pending,
    InTransit { vehicle: &'static str },
    Delivered { left_at: Option<&'static str> },
    Delayed { reason: &'static str },
}

fn describe(code: &str, shipment: &Shipment) -> String {
    let detail = match shipment {
        Shipment::Pending => "waiting for pickup".to_string(),
        Shipment::InTransit { vehicle } => format!("in transit via {}", vehicle),
        Shipment::Delivered { left_at } => match left_at {
            Some(place) => format!("delivered to {}", place),
            None => "delivered".to_string(),
        },
        Shipment::Delayed { .. } => "delivery exception".to_string(),
    };

    format!("{} {}", code, detail)
}

fn main() {
    let shipments = [
        ("A1", Shipment::Pending),
        ("B2", Shipment::InTransit { vehicle: "truck" }),
        ("C3", Shipment::Delivered { left_at: Some("mailbox") }),
        ("D4", Shipment::Delayed { reason: "weather" }),
    ];

    for (idx, (code, shipment)) in shipments.iter().enumerate() {
        if idx > 0 {
            println!();
        }
        print!("{}", describe(code, shipment));
    }
}
