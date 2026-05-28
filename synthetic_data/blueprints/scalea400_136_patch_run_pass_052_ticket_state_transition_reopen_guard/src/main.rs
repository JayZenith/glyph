#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    New,
    InProgress,
    Closed,
}

#[derive(Clone, Copy)]
enum Event {
    Start,
    Resolve,
    Reopen,
}

fn apply(mut status: Status, events: &[Event]) -> Status {
    for event in events {
        status = match (status, event) {
            (Status::New, Event::Start) => Status::InProgress,
            (Status::InProgress, Event::Resolve) => Status::Closed,
            (_, Event::Reopen) => Status::InProgress,
            _ => status,
        };
    }
    status
}

fn label(status: Status) -> &'static str {
    match status {
        Status::New => "New",
        Status::InProgress => "InProgress",
        Status::Closed => "Closed",
    }
}

fn main() {
    let tickets = [
        ("A", Status::New, vec![Event::Start, Event::Resolve, Event::Reopen]),
        ("B", Status::InProgress, vec![Event::Resolve, Event::Reopen, Event::Resolve]),
        ("C", Status::Closed, vec![Event::Start, Event::Resolve]),
    ];

    for (id, initial, events) in tickets {
        let final_status = apply(initial, &events);
        println!("{}:{}", id, label(final_status));
    }
}
