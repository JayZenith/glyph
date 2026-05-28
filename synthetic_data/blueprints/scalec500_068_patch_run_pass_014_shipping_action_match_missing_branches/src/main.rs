enum ShipmentStatus {
    Pending,
    Packed,
    Shipped,
    Returned,
    Cancelled,
}

fn action(status: ShipmentStatus) -> &'static str {
    match status {
        ShipmentStatus::Pending | ShipmentStatus::Packed => "queue",
        ShipmentStatus::Shipped => "dispatch",
        ShipmentStatus::Returned | ShipmentStatus::Cancelled => "notify",
    }
}

fn label(status: &ShipmentStatus) -> &'static str {
    match status {
        ShipmentStatus::Pending => "pending",
        ShipmentStatus::Packed => "packed",
        ShipmentStatus::Shipped => "shipped",
        ShipmentStatus::Returned => "returned",
        ShipmentStatus::Cancelled => "cancelled",
    }
}

fn main() {
    let statuses = [
        ShipmentStatus::Pending,
        ShipmentStatus::Packed,
        ShipmentStatus::Shipped,
        ShipmentStatus::Returned,
        ShipmentStatus::Cancelled,
    ];

    for status in statuses {
        println!("{} -> {}", label(&status), action(status));
    }
}
