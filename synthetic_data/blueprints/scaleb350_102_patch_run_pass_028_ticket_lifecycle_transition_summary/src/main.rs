use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Status {
    Backlog,
    InProgress,
    Blocked,
    Done,
}

#[derive(Clone, Copy, Debug)]
enum Event {
    Start,
    Block,
    Unblock,
    Complete,
    Reopen,
}

fn apply(status: Status, event: Event) -> (Status, bool) {
    match event {
        Event::Start => (Status::InProgress, true),
        Event::Block => (Status::Blocked, true),
        Event::Unblock => (Status::InProgress, true),
        Event::Complete => (Status::Done, true),
        Event::Reopen => (Status::Backlog, true),
    }
}

fn label(status: Status) -> &'static str {
    match status {
        Status::Backlog => "Backlog",
        Status::InProgress => "InProgress",
        Status::Blocked => "Blocked",
        Status::Done => "Done",
    }
}

fn main() {
    let data = [
        ("A", vec![Event::Start, Event::Block, Event::Unblock, Event::Complete]),
        ("B", vec![Event::Start, Event::Complete, Event::Reopen, Event::Start]),
        ("C", vec![Event::Block, Event::Unblock, Event::Start, Event::Block]),
        ("D", vec![Event::Complete, Event::Reopen]),
        ("E", vec![Event::Start, Event::Complete]),
    ];

    let mut counts: BTreeMap<Status, usize> = BTreeMap::new();
    let mut invalid = 0usize;
    let mut lines = Vec::new();

    for (name, events) in data {
        let mut status = Status::Backlog;
        for event in events {
            let (next, ok) = apply(status, event);
            if !ok {
                invalid += 1;
            }
            status = next;
        }
        *counts.entry(status).or_insert(0) += 1;
        lines.push(format!("{}: {}", name, label(status)));
    }

    let done = counts.get(&Status::Done).copied().unwrap_or(0);
    let in_progress = counts.get(&Status::InProgress).copied().unwrap_or(0);
    let blocked = counts.get(&Status::Blocked).copied().unwrap_or(0);
    let backlog = counts.get(&Status::Backlog).copied().unwrap_or(0);

    println!("done: {}", done);
    println!("in_progress: {}", in_progress);
    println!("blocked: {}", blocked);
    println!("backlog: {}", backlog);
    println!("invalid: {}", invalid);
    for line in lines {
        println!("{}", line);
    }
}
