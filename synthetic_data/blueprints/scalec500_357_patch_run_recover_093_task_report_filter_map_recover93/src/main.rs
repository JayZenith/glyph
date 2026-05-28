struct Task {
    owner: &'static str,
    title: &'static str,
    done: bool,
    score: i32,
}

fn main() {
    let tasks = vec![
        Task { owner: "Ava", title: "alpha", done: false, score: 3 },
        Task { owner: "Ava", title: "beta", done: true, score: 5 },
        Task { owner: "Ava", title: "gamma", done: false, score: 1 },
        Task { owner: "Ben", title: "delta", done: false, score: 0 },
        Task { owner: "Cleo", title: "epsilon", done: false, score: 2 },
        Task { owner: "Cleo", title: "zeta", done: false, score: -1 },
    ];

    let owners = ["Ava", "Ben", "Cleo"];

    let lines: Vec<String> = owners
        .iter()
        .filter_map(|owner| {
            let titles: Vec<&str> = tasks
                .iter()
                .filter(|t| t.owner == *owner && (!t.done || t.score > 0))
                .map(|t| t.title)
                .collect();

            if titles.is_empty() {
                None
            } else {
                Some(format!("{}:{}", owner, titles.join(",")))
            }
        })
        .collect();

    println!("{}", lines.join("\n"));
}
