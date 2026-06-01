enum ShipmentStatus {
    Pending,
    Moving { hub: &'static str },
    Delayed { reason: &'static str },
    Delivered { attempts: u8 },
    Returned,
}

struct Shipment {
    code: &'static str,
    status: ShipmentStatus,
}

fn render_status(status: &ShipmentStatus) -> String {
    match status {
        ShipmentStatus::Pending => "queued [pending]".to_string(),
        ShipmentStatus::Moving { hub } => format!("moving via {} [active]", hub),
        ShipmentStatus::Delayed { .. } => "delayed [attention]".to_string(),
        ShipmentStatus::Delivered { .. } => "delivered [done]".to_string(),
        ShipmentStatus::Returned => "returned [closed]".to_string(),
    }
}

fn main() {
    let shipments = [
        Shipment {
            code: "PKG-17",
            status: ShipmentStatus::Pending,
        },
        Shipment {
            code: "PKG-18",
            status: ShipmentStatus::Moving { hub: "North Hub" },
        },
        Shipment {
            code: "PKG-19",
            status: ShipmentStatus::Delayed { reason: "weather" },
        },
        Shipment {
            code: "PKG-20",
            status: ShipmentStatus::Delivered { attempts: 2 },
        },
        Shipment {
            code: "PKG-21",
            status: ShipmentStatus::Returned,
        },
    ];

    for shipment in shipments {
        println!("{}: {}", shipment.code, render_status(&shipment.status));
    }
}
