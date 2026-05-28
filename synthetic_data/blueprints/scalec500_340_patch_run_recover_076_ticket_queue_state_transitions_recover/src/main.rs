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
            Status::Open => "open",
            Status::InProgress => "in_progress",
            Status::Closed => "closed",
        }
    }
}

fn main() {
    let events = [
        ("A", "start"),
        ("B", "start"),
        ("B", "close"),
        ("C", "start"),
        ("A", "close"),
        ("B", "reopen"),
    ];

    let mut tickets: BTreeMap<&str, Status> = BTreeMap::new();

    for (id, event) in events {
        let status = tickets.entry(id).or_insert(Status::Open);
        match event {
            "start" => *status = Status::Open,
            "close" => *status = Status::Closed,
            "reopen" => *status = Status::InProgress,
            _ => {}
        }
    }

    let mut out = String::new();
    for (id, status) in tickets {
        out.push_str(id);
        out.push(':');
        out.push_str(status.as_str());
        out.push('\n');
    }
    print!("{}", out);
}
