use std::collections::BTreeMap;

fn main() {
    let events = [
        ("A", "create"),
        ("B", "create"),
        ("A", "start"),
        ("C", "create"),
        ("A", "finish"),
        ("B", "cancel"),
    ];

    let mut states: BTreeMap<&str, &str> = BTreeMap::new();

    for (id, event) in events {
        match event {
            "create" => {
                states.insert(id, "new");
            }
            "start" => {
                states.insert(id, "running");
            }
            "finish" => {
                states.insert(id, "done");
            }
            "cancel" => {
                states.insert(id, "done");
            }
            _ => {}
        }
    }

    let mut out = Vec::new();
    for (id, state) in states {
        out.push(format!("{}:{}", id, state));
    }
    print!("{}", out.join("\n"));
}
