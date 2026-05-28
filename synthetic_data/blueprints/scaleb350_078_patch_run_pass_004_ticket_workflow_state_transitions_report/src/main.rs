use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    Todo,
    InProgress,
    Blocked,
    Closed,
}

#[derive(Clone, Debug)]
struct Ticket {
    owner: Option<&'static str>,
    status: Status,
    blockers: u32,
}

enum Event {
    Assign(&'static str, &'static str),
    Start(&'static str),
    Block(&'static str),
    Unblock(&'static str),
    Close(&'static str),
}

fn apply(ticket: &mut Ticket, event: &Event) {
    match *event {
        Event::Assign(_, owner) => {
            ticket.owner = Some(owner);
        }
        Event::Start(_) => {
            ticket.status = Status::InProgress;
        }
        Event::Block(_) => {
            ticket.blockers += 1;
            ticket.status = Status::Blocked;
        }
        Event::Unblock(_) => {
            if ticket.blockers > 0 {
                ticket.blockers -= 1;
            }
            ticket.status = Status::InProgress;
        }
        Event::Close(_) => {
            ticket.status = Status::Closed;
        }
    }
}

fn main() {
    let mut tickets = BTreeMap::from([
        (
            "T1",
            Ticket {
                owner: None,
                status: Status::Todo,
                blockers: 0,
            },
        ),
        (
            "T2",
            Ticket {
                owner: None,
                status: Status::Todo,
                blockers: 0,
            },
        ),
        (
            "T3",
            Ticket {
                owner: None,
                status: Status::Todo,
                blockers: 0,
            },
        ),
    ]);

    let events = [
        Event::Assign("T1", "alice"),
        Event::Start("T1"),
        Event::Close("T1"),
        Event::Assign("T2", "bob"),
        Event::Start("T2"),
        Event::Block("T2"),
        Event::Assign("T3", "carol"),
        Event::Start("T3"),
        Event::Block("T3"),
        Event::Unblock("T3"),
    ];

    for event in &events {
        let id = match *event {
            Event::Assign(id, _)
            | Event::Start(id)
            | Event::Block(id)
            | Event::Unblock(id)
            | Event::Close(id) => id,
        };
        apply(tickets.get_mut(id).unwrap(), event);
    }

    for (id, ticket) in tickets {
        let owner = ticket.owner.unwrap_or("-");
        let status = match ticket.status {
            Status::Todo => "Todo",
            Status::InProgress => "InProgress",
            Status::Blocked => "Blocked",
            Status::Closed => "Closed",
        };
        println!("{id}:{status} owner={owner} blockers={}", ticket.blockers);
    }
}
