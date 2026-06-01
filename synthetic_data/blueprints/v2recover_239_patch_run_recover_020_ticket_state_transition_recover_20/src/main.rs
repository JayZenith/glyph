use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum Event {
    Create(u32),
    Start(u32),
    Resolve(u32),
    Reopen(u32),
}

fn main() {
    let events = [
        Event::Create(1),
        Event::Start(1),
        Event::Resolve(1),
        Event::Reopen(1),
        Event::Create(2),
        Event::Resolve(2),
        Event::Create(3),
        Event::Start(3),
    ];

    let mut tickets: BTreeMap<u32, &'static str> = BTreeMap::new();

    for event in events {
        match event {
            Event::Create(id) => {
                tickets.insert(id, "new");
            }
            Event::Start(id) => {
                if tickets.contains_key(&id) {
                    tickets.insert(id, "open");
                }
            }
            Event::Resolve(id) => {
                if tickets.contains_key(&id) {
                    tickets.insert(id, "closed");
                }
            }
            Event::Reopen(id) => {
                if tickets.contains_key(&id) {
                    tickets.insert(id, "new");
                }
            }
        }
    }

    let mut lines = Vec::new();
    for (id, state) in tickets {
        lines.push(format!("T{}:{}", id, state));
    }
    println!("{}", lines.join("\n"));
}
