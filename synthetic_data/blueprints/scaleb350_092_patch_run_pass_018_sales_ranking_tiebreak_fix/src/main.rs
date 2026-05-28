struct Entry {
    name: &'static str,
    score: u32,
    penalty: u32,
}

fn main() {
    let mut items = vec![
        Entry { name: "Zoe", score: 12, penalty: 2 },
        Entry { name: "Ava", score: 12, penalty: 1 },
        Entry { name: "Liam", score: 10, penalty: 0 },
        Entry { name: "Mia", score: 12, penalty: 1 },
    ];

    items.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .reverse()
            .then_with(|| b.penalty.cmp(&a.penalty))
            .then_with(|| b.name.cmp(a.name))
    });

    for (idx, item) in items.iter().enumerate() {
        println!(
            "{}. {} score={} penalty={}",
            idx + 1,
            item.name,
            item.score,
            item.penalty
        );
    }
}
