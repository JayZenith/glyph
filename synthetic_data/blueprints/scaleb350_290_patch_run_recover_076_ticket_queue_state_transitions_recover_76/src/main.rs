use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    New,
    Open,
    Blocked,
    Closed,
}

#[derive(Clone, Debug)]
struct Ticket {
    status: Status,
    points: i32,
}

enum Event {
    Create { id: &'static str, points: i32 },
    Start { id: &'static str },
    Block { id: &'static str },
    Resume { id: &'static str },
    AddPoints { id: &'static str, points: i32 },
    Resolve { id: &'static str },
    Reopen { id: &'static str },
}

fn status_name(s: Status) -> &'static str {
    match s {
        Status::New => "new",
        Status::Open => "open",
        Status::Blocked => "blocked",
        Status::Closed => "closed",
    }
}

fn main() {
    let events = vec![
        Event::Create { id: "A", points: 3 },
        Event::Start { id: "A" },
        Event::AddPoints { id: "A", points: 2 },
        Event::Resolve { id: "A" },
        Event::Reopen { id: "A" },
        Event::AddPoints { id: "A", points: 1 },
        Event::Resolve { id: "A" },
        Event::Create { id: "B", points: 2 },
        Event::Start { id: "B" },
        Event::Block { id: "B" },
        Event::Resume { id: "B" },
        Event::AddPoints { id: "B", points: 1 },
        Event::Create { id: "C", points: 5 },
        Event::Resolve { id: "C" },
        Event::Reopen { id: "C" },
        Event::Start { id: "C" },
        Event::AddPoints { id: "C", points: 1 },
        Event::Resolve { id: "C" },
    ];

    let mut tickets: BTreeMap<&'static str, Ticket> = BTreeMap::new();

    for event in events {
        match event {
            Event::Create { id, points } => {
                tickets.insert(
                    id,
                    Ticket {
                        status: Status::New,
                        points,
                    },
                );
            }
            Event::Start { id } => {
                if let Some(ticket) = tickets.get_mut(id) {
                    ticket.status = Status::Open;
                }
            }
            Event::Block { id } => {
                if let Some(ticket) = tickets.get_mut(id) {
                    ticket.status = Status::Blocked;
                }
            }
            Event::Resume { id } => {
                if let Some(ticket) = tickets.get_mut(id) {
                    ticket.status = Status::New;
                }
            }
            Event::AddPoints { id, points } => {
                if let Some(ticket) = tickets.get_mut(id) {
                    ticket.points += points;
                }
            }
            Event::Resolve { id } => {
                if let Some(ticket) = tickets.get_mut(id) {
                    if ticket.status == Status::Open {
                        ticket.status = Status::Closed;
                    }
                }
            }
            Event::Reopen { id } => {
                if let Some(ticket) = tickets.get_mut(id) {
                    ticket.status = Status::New;
                    ticket.points -= 1;
                }
            }
        }
    }

    let mut out = Vec::new();
    for (id, ticket) in tickets {
        out.push(format!("{}:{}:{}", id, status_name(ticket.status), ticket.points));
    }
    println!("{}", out.join("\n"));
}
