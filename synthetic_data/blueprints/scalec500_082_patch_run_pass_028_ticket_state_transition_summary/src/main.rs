use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    New,
    InProgress,
    Blocked,
    Closed,
}

impl Status {
    fn label(self) -> &'static str {
        match self {
            Status::New => "new",
            Status::InProgress => "in_progress",
            Status::Blocked => "blocked",
            Status::Closed => "closed",
        }
    }

    fn is_open(self) -> bool {
        !matches!(self, Status::Closed)
    }
}

enum Event<'a> {
    Create(&'a str),
    Start(&'a str),
    Block(&'a str),
    Resume(&'a str),
    Resolve(&'a str),
    Reopen(&'a str),
}

fn apply_event(states: &mut BTreeMap<String, Status>, event: Event<'_>) {
    match event {
        Event::Create(id) => {
            states.entry(id.to_string()).or_insert(Status::New);
        }
        Event::Start(id) => {
            if states.contains_key(id) {
                states.insert(id.to_string(), Status::InProgress);
            }
        }
        Event::Block(id) => {
            if states.contains_key(id) {
                states.insert(id.to_string(), Status::Blocked);
            }
        }
        Event::Resume(id) => {
            if let Some(status) = states.get_mut(id) {
                if *status == Status::Blocked {
                    *status = Status::New;
                }
            }
        }
        Event::Resolve(id) => {
            if states.contains_key(id) {
                states.insert(id.to_string(), Status::Closed);
            }
        }
        Event::Reopen(id) => {
            if states.contains_key(id) {
                states.insert(id.to_string(), Status::Blocked);
            }
        }
    }
}

fn main() {
    let events = [
        Event::Create("A1"),
        Event::Start("A1"),
        Event::Resolve("A1"),
        Event::Reopen("A1"),
        Event::Start("A1"),
        Event::Resolve("A1"),
        Event::Create("B7"),
        Event::Start("B7"),
        Event::Block("B7"),
        Event::Resume("B7"),
        Event::Resolve("B7"),
        Event::Create("C3"),
        Event::Start("C3"),
        Event::Block("C3"),
        Event::Resume("C3"),
        Event::Create("D9"),
        Event::Resolve("D9"),
        Event::Reopen("D9"),
    ];

    let mut states = BTreeMap::new();
    for event in events {
        apply_event(&mut states, event);
    }

    let mut open_count = 0;
    let mut closed_count = 0;
    for (id, status) in &states {
        println!("{}={}", id, status.label());
        if status.is_open() {
            open_count += 1;
        } else {
            closed_count += 1;
        }
    }
    println!("open_count={}", open_count);
    println!("closed_count={}", closed_count);
}
