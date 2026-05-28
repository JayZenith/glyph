use std::collections::BTreeMap;

#[derive(Clone, Copy)]
enum State {
    Todo,
    InProgress,
    Blocked,
    Done,
}

impl State {
    fn as_str(self) -> &'static str {
        match self {
            State::Todo => "todo",
            State::InProgress => "in_progress",
            State::Blocked => "blocked",
            State::Done => "done",
        }
    }
}

fn main() {
    let events = [
        ("alpha", "start"),
        ("beta", "start"),
        ("beta", "block"),
        ("beta", "resume"),
        ("alpha", "finish"),
        ("gamma", "block"),
    ];

    let mut states: BTreeMap<&str, State> = BTreeMap::new();

    for (task, event) in events {
        let next = match event {
            "start" => State::InProgress,
            "block" => State::Done,
            "resume" => State::Todo,
            "finish" => State::Done,
            _ => State::Todo,
        };
        states.insert(task, next);
    }

    let mut out = Vec::new();
    for (task, state) in states {
        out.push(format!("{}:{}", task, state.as_str()));
    }
    print!("{}", out.join("\n"));
}
