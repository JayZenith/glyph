struct Task {
    name: &'static str,
    points: i32,
    done: bool,
    blocked: bool,
}

fn main() {
    let tasks = vec![
        Task { name: "alpha", points: 3, done: false, blocked: false },
        Task { name: "beta", points: 0, done: false, blocked: false },
        Task { name: "gamma", points: 5, done: true, blocked: false },
        Task { name: "delta", points: 2, done: false, blocked: true },
        Task { name: "x-secret", points: 8, done: false, blocked: false },
        Task { name: "epsilon", points: 5, done: false, blocked: false },
    ];

    let kept: Vec<(&str, i32)> = tasks
        .iter()
        .filter(|t| !t.done)
        .filter(|t| t.points >= 0)
        .map(|t| (t.name, t.points))
        .collect();

    let names = kept
        .iter()
        .map(|(name, _)| name.to_uppercase())
        .collect::<Vec<_>>()
        .join(",");

    let total: i32 = kept.iter().map(|(_, points)| points).sum();

    println!("{} | total={}", names, total);
}
