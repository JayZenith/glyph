struct Task {
    owner: &'static str,
    active: bool,
    score: i32,
    tags: &'static [&'static str],
}

fn main() {
    let tasks = vec![
        Task { owner: "ann", active: true, score: 5, tags: &["core"] },
        Task { owner: "ann", active: true, score: -2, tags: &["ops"] },
        Task { owner: "ann", active: true, score: 2, tags: &["misc", "ops"] },
        Task { owner: "bob", active: false, score: 9, tags: &["core"] },
        Task { owner: "bob", active: true, score: 7, tags: &["ops"] },
        Task { owner: "bob", active: true, score: 3, tags: &["misc"] },
    ];

    let owners = ["ann", "bob"];
    let mut lines = Vec::new();
    let mut total = 0;

    for owner in owners {
        let sum: i32 = tasks
            .iter()
            .filter(|t| t.owner == owner)
            .filter(|t| t.active)
            .map(|t| t.score)
            .sum();
        total += sum;
        lines.push(format!("{}={}", owner, sum));
    }

    lines.push(format!("total={}", total));
    println!("{}", lines.join("\n"));
}
