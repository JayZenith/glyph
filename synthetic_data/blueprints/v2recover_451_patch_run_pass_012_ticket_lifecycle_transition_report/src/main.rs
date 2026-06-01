use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Open,
    Active,
    Paused,
    Resolved,
    Closed,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Open => "open",
            Status::Active => "active",
            Status::Paused => "paused",
            Status::Resolved => "resolved",
            Status::Closed => "closed",
        }
    }
}

struct Ticket {
    status: Status,
    owner: Option<&'static str>,
    pause_reason: Option<&'static str>,
    history: Vec<String>,
}

impl Ticket {
    fn new() -> Self {
        Self {
            status: Status::Open,
            owner: None,
            pause_reason: None,
            history: vec!["created".to_string()],
        }
    }
}

enum Event {
    Create { id: &'static str },
    Assign { id: &'static str, user: &'static str },
    Pause { id: &'static str, reason: &'static str },
    Resume { id: &'static str },
    Resolve { id: &'static str },
    Close { id: &'static str },
}

fn apply_event(tickets: &mut BTreeMap<&'static str, Ticket>, event: Event) {
    match event {
        Event::Create { id } => {
            tickets.entry(id).or_insert_with(Ticket::new);
        }
        Event::Assign { id, user } => {
            if let Some(ticket) = tickets.get_mut(id) {
                ticket.owner = Some(user);
                ticket.status = Status::Active;
                ticket.history.push(format!("assigned({})", user));
            }
        }
        Event::Pause { id, reason } => {
            if let Some(ticket) = tickets.get_mut(id) {
                ticket.status = Status::Paused;
                ticket.pause_reason = Some(reason);
                ticket.history.push(format!("paused({})", reason));
            }
        }
        Event::Resume { id } => {
            if let Some(ticket) = tickets.get_mut(id) {
                ticket.status = Status::Open;
                ticket.history.push("active".to_string());
            }
        }
        Event::Resolve { id } => {
            if let Some(ticket) = tickets.get_mut(id) {
                ticket.status = Status::Resolved;
                ticket.history.push("resolved".to_string());
            }
        }
        Event::Close { id } => {
            if let Some(ticket) = tickets.get_mut(id) {
                ticket.status = Status::Closed;
                ticket.history.push("closed".to_string());
            }
        }
    }
}

fn main() {
    let mut tickets = BTreeMap::new();
    let events = vec![
        Event::Create { id: "T1" },
        Event::Assign { id: "T1", user: "alice" },
        Event::Pause { id: "T1", reason: "waiting_customer" },
        Event::Resume { id: "T1" },
        Event::Resolve { id: "T1" },
        Event::Close { id: "T1" },
        Event::Create { id: "T2" },
        Event::Assign { id: "T2", user: "bob" },
        Event::Pause { id: "T2", reason: "waiting_parts" },
        Event::Resume { id: "T2" },
        Event::Create { id: "T3" },
        Event::Resolve { id: "T3" },
    ];

    for event in events {
        apply_event(&mut tickets, event);
    }

    let mut open = 0;
    let mut paused = 0;
    let mut resolved = 0;
    let mut closed = 0;

    for (id, ticket) in &tickets {
        match ticket.status {
            Status::Open | Status::Active => open += 1,
            Status::Paused => paused += 1,
            Status::Resolved => resolved += 1,
            Status::Closed => closed += 1,
        }

        let owner = ticket.owner.unwrap_or("none");
        println!(
            "{}:{}:owner={}:hist={}",
            id,
            ticket.status.as_str(),
            owner,
            ticket.history.join(">")
        );
    }

    println!(
        "summary open={} paused={} resolved={} closed={}",
        open, paused, resolved, closed
    );
}
