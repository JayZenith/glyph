struct Task {
    name: &'static str,
    active: bool,
    score: i32,
}

fn main() {
    let tasks = vec![
        Task { name: "a", active: true, score: 4 },
        Task { name: "b", active: false, score: 6 },
        Task { name: "c", active: true, score: 8 },
        Task { name: "d", active: true, score: 5 },
    ];

    let out = tasks
        .iter()
        .filter(|t| t.active)
        .filter(|t| t.score % 2 == 1)
        .map(|t| format!("{}:{}", t.name, t.score))
        .collect::<Vec<_>>()
        .join(",");

    print!("{}", out);
}
