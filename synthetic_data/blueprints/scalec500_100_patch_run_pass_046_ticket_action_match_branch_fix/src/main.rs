enum Ticket {
    Open { id: &'static str, urgent: bool },
    Closed { id: &'static str, refunded: bool },
    Pending { id: &'static str, attempts: u8 },
}

fn action(ticket: &Ticket) -> &'static str {
    match ticket {
        Ticket::Open { urgent: true, .. } => "queue",
        Ticket::Open { urgent: false, .. } => "review",
        Ticket::Closed { refunded: true, .. } => "archive",
        Ticket::Closed { refunded: false, .. } => "ship",
        Ticket::Pending { attempts, .. } if *attempts >= 3 => "investigate",
        Ticket::Pending { .. } => "queue",
    }
}

fn id(ticket: &Ticket) -> &'static str {
    match ticket {
        Ticket::Open { id, .. } | Ticket::Closed { id, .. } | Ticket::Pending { id, .. } => id,
    }
}

fn main() {
    let tickets = [
        Ticket::Pending { id: "A12", attempts: 1 },
        Ticket::Pending { id: "B07", attempts: 3 },
        Ticket::Closed { id: "C99", refunded: false },
        Ticket::Closed { id: "D20", refunded: true },
        Ticket::Open { id: "E11", urgent: false },
    ];

    for ticket in tickets.iter() {
        println!("{}: {}", id(ticket), action(ticket));
    }
}
