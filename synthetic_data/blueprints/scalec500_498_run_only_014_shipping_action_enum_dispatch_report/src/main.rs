enum Fulfillment {
    Digital,
    Physical { weight_kg: f32 },
    Pickup { store_id: u16 },
}

enum Payment {
    Card { cents: u32, verified: bool },
    Invoice { due_days: u8 },
    Failed,
}

enum OrderEvent {
    Create { items: Vec<&'static str> },
    Pay(Payment),
    Ship(Fulfillment),
    Cancel { reason: &'static str },
}

fn describe(event: &OrderEvent) -> String {
    match event {
        OrderEvent::Create { items } => format!("reserve stock for {} sku(s)", items.len()),
        OrderEvent::Pay(payment) => match payment {
            Payment::Card { cents, verified } if *verified && *cents <= 10_000 => {
                format!("charge card {} cents", cents)
            }
            Payment::Card { cents, .. } => format!("manual review due to amount {}", cents),
            Payment::Invoice { due_days } => format!("issue invoice due in {} days", due_days),
            Payment::Failed => "payment failed".to_string(),
        },
        OrderEvent::Ship(mode) => match mode {
            Fulfillment::Digital => "skip digital shipment".to_string(),
            Fulfillment::Physical { weight_kg } if *weight_kg > 0.0 => {
                format!("send parcel {:.2} kg", weight_kg)
            }
            Fulfillment::Physical { .. } => "hold invalid package".to_string(),
            Fulfillment::Pickup { store_id } => format!("notify pickup at store {}", store_id),
        },
        OrderEvent::Cancel { reason } => format!("cancel order by {}", reason),
    }
}

fn main() {
    let events = vec![
        OrderEvent::Pay(Payment::Card {
            cents: 4_999,
            verified: true,
        }),
        OrderEvent::Create {
            items: vec!["BOOK-1", "GAME-2"],
        },
        OrderEvent::Ship(Fulfillment::Digital),
        OrderEvent::Ship(Fulfillment::Physical { weight_kg: 3.25 }),
        OrderEvent::Cancel {
            reason: "user request",
        },
        OrderEvent::Pay(Payment::Card {
            cents: 12_000,
            verified: false,
        }),
    ];

    for (idx, event) in events.iter().enumerate() {
        println!("{}: {}", idx + 1, describe(event));
    }
}
