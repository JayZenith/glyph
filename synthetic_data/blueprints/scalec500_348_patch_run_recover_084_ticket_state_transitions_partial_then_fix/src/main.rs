fn apply_event(state: &str, event: &str) -> String {
    match (state, event) {
        ("Open", "start") => "InProgress".to_string(),
        ("InProgress", "resolve") => "Resolved".to_string(),
        (_, "close") => "Closed".to_string(),
        (s, _) => s.to_string(),
    }
}

fn main() {
    let logs = [
        ("A", vec!["start", "resolve", "close"]),
        ("B", vec!["start", "resolve"]),
        ("C", vec!["close"]),
    ];

    let mut lines = Vec::new();
    for (id, events) in logs {
        let mut state = "Open".to_string();
        for event in events {
            state = apply_event(&state, event);
        }
        lines.push(format!("{}:{}", id, state));
    }

    println!("{}", lines.join("\n"));
}
