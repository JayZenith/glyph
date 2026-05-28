use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum Status {
    Open,
    InProgress,
    Closed,
}

enum Event {
    Start,
    Close,
    Reopen,
}

fn apply_event(status: Status, event: Event) -> Status {
    match event {
        Event::Start => Status::InProgress,
        Event::Close => Status::Open,
        Event::Reopen => status,
    }
}

fn label(status: Status) -> &'static str {
    match status {
        Status::Open => "Open",
        Status::InProgress => "InProgress",
        Status::Closed => "Closed",
    }
}

fn main() {
    let mut tickets = BTreeMap::new();
    tickets.insert("A", vec![Event::Start, Event::Close]);
    tickets.insert("B", vec![Event::Start]);
    tickets.insert("C", vec![Event::Close, Event::Reopen]);

    let mut out = Vec::new();
    for (id, events) in tickets {
        let mut status = Status::Open;
        for event in events {
            status = apply_event(status, event);
        }
        out.push(format!("{}: {}", id, label(status)));
    }

    println!("{}", out.join("\n"));
}
