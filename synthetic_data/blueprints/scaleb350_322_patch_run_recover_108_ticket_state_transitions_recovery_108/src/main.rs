use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum State {
    Open,
    Investigating,
    Resolved,
    Closed,
}

impl State {
    fn as_str(self) -> &'static str {
        match self {
            State::Open => "Open",
            State::Investigating => "Investigating",
            State::Resolved => "Resolved",
            State::Closed => "Closed",
        }
    }
}

struct Ticket {
    state: State,
    owner: &'static str,
    ack: bool,
    sev: u8,
}

enum Event {
    New(&'static str, &'static str, u8),
    Ack(&'static str),
    Start(&'static str),
    Resolve(&'static str),
    Reopen(&'static str),
    Close(&'static str),
    Escalate(&'static str),
}

fn apply(tickets: &mut BTreeMap<&'static str, Ticket>, ev: Event) {
    match ev {
        Event::New(id, owner, sev) => {
            tickets.insert(
                id,
                Ticket {
                    state: State::Open,
                    owner,
                    ack: false,
                    sev,
                },
            );
        }
        Event::Ack(id) => {
            if let Some(t) = tickets.get_mut(id) {
                t.ack = true;
            }
        }
        Event::Start(id) => {
            if let Some(t) = tickets.get_mut(id) {
                t.state = State::Investigating;
            }
        }
        Event::Resolve(id) => {
            if let Some(t) = tickets.get_mut(id) {
                t.state = State::Resolved;
            }
        }
        Event::Reopen(id) => {
            if let Some(t) = tickets.get_mut(id) {
                t.state = State::Open;
                t.ack = false;
            }
        }
        Event::Close(id) => {
            if let Some(t) = tickets.get_mut(id) {
                t.state = State::Closed;
            }
        }
        Event::Escalate(id) => {
            if let Some(t) = tickets.get_mut(id) {
                t.sev += 1;
            }
        }
    }
}

fn main() {
    let events = [
        Event::New("T1", "alice", 2),
        Event::Ack("T1"),
        Event::Start("T1"),
        Event::Resolve("T1"),
        Event::Close("T1"),
        Event::New("T2", "bob", 1),
        Event::Start("T2"),
        Event::Escalate("T2"),
        Event::Escalate("T2"),
        Event::Resolve("T2"),
        Event::New("T3", "carol", 2),
        Event::Ack("T3"),
        Event::Start("T3"),
        Event::Resolve("T3"),
        Event::Reopen("T3"),
    ];

    let mut tickets = BTreeMap::new();
    for ev in events {
        apply(&mut tickets, ev);
    }

    let mut out = Vec::new();
    for (id, t) in tickets {
        out.push(format!(
            "{}:{} owner={} ack={} sev={}",
            id,
            t.state.as_str(),
            t.owner,
            t.ack,
            t.sev
        ));
    }
    print!("{}", out.join("\n"));
}
