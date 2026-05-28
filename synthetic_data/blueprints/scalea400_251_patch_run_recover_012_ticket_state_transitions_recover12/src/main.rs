use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum Status {
    Open,
    InProgress,
    Closed,
}

impl Status {
    fn as_str(self) -> &'static str {
        match self {
            Status::Open => "Open",
            Status::InProgress => "InProgress",
            Status::Closed => "Closed",
        }
    }
}

enum Event {
    Create(&'static str),
    Start(&'static str),
    Resolve(&'static str),
    Reopen(&'static str),
}

fn main() {
    let events = [
        Event::Create("T1"),
        Event::Start("T1"),
        Event::Resolve("T1"),
        Event::Create("T2"),
        Event::Resolve("T2"),
        Event::Reopen("T2"),
        Event::Create("T3"),
        Event::Start("T3"),
    ];

    let mut tickets: BTreeMap<&str, Status> = BTreeMap::new();

    for event in events {
        match event {
            Event::Create(id) => {
                tickets.insert(id, Status::Open);
            }
            Event::Start(id) => {
                tickets.insert(id, Status::InProgress);
            }
            Event::Resolve(id) => {
                tickets.insert(id, Status::Open);
            }
            Event::Reopen(id) => {
                tickets.insert(id, Status::Closed);
            }
        }
    }

    for (id, status) in tickets {
        println!("{}: {}", id, status.as_str());
    }
}
