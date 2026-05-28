#[derive(Clone, Debug)]
struct Entry {
    name: &'static str,
    score: u32,
    attempts: u32,
}

fn leaderboard(mut items: Vec<Entry>) -> String {
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| b.attempts.cmp(&a.attempts))
            .then_with(|| b.name.cmp(a.name))
    });

    let mut out = Vec::new();
    let mut rank = 1usize;

    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            let prev = &items[i - 1];
            if item.score != prev.score || item.attempts != prev.attempts {
                rank = i + 1;
            }
        }
        out.push(format!(
            "{}. {} score={} attempts={}",
            rank, item.name, item.score, item.attempts
        ));
    }

    out.join("\n")
}

fn main() {
    let items = vec![
        Entry { name: "zoe", score: 88, attempts: 1 },
        Entry { name: "ava", score: 97, attempts: 2 },
        Entry { name: "liam", score: 91, attempts: 2 },
        Entry { name: "ivy", score: 97, attempts: 1 },
        Entry { name: "emma", score: 91, attempts: 4 },
        Entry { name: "noah", score: 91, attempts: 2 },
    ];

    println!("{}", leaderboard(items));
}
