use std::collections::BTreeMap;

#[derive(Clone, Default)]
struct Ticket {
    owner: String,
    state: String,
}

fn main() {
    let events = [
        "A create alice",
        "A start bob",
        "A reassign alice",
        "A close",
        "B create bob",
        "B start bob",
        "B block infra",
        "B start bob",
        "C create carol",
        "C block waiting_parts",
        "C reassign dana",
    ];

    let mut tickets: BTreeMap<&str, Ticket> = BTreeMap::new();

    for line in events {
        let mut parts = line.split_whitespace();
        let id = parts.next().unwrap();
        let action = parts.next().unwrap();
        let arg = parts.next();

        let entry = tickets.entry(id).or_default();
        match action {
            "create" => {
                entry.owner = arg.unwrap_or("unassigned").to_string();
                entry.state = "new".to_string();
            }
            "start" => {
                entry.state = "in_progress".to_string();
                if let Some(owner) = arg {
                    entry.owner = owner.to_string();
                }
            }
            "block" => {
                entry.state = "blocked".to_string();
            }
            "reassign" => {
                entry.owner = arg.unwrap_or("unassigned").to_string();
                entry.state = "new".to_string();
            }
            "close" => {
                entry.state = "done".to_string();
            }
            _ => {}
        }
    }

    let mut out = Vec::new();
    for (id, t) in tickets {
        out.push(format!("{}: {} [owner={}]", id, t.state, t.owner));
    }
    println!("{}", out.join("\n"));
}
