struct Entry {
    name: &'static str,
    score: u32,
    penalties: u32,
}

fn main() {
    let mut entries = vec![
        Entry { name: "Zoe", score: 12, penalties: 3 },
        Entry { name: "Ava", score: 15, penalties: 1 },
        Entry { name: "Mia", score: 15, penalties: 2 },
        Entry { name: "Ben", score: 12, penalties: 0 },
        Entry { name: "Eve", score: 15, penalties: 1 },
    ];

    entries.sort_by(|a, b| {
        a.score.cmp(&b.score)
            .then(a.penalties.cmp(&b.penalties))
            .then(a.name.cmp(&b.name))
    });

    let mut lines = Vec::new();
    for (i, e) in entries.iter().enumerate() {
        let rank = if i > 0
            && entries[i - 1].score == e.score
            && entries[i - 1].penalties == e.penalties
        {
            i
        } else {
            i + 1
        };
        lines.push(format!("{}. {} {} {}", rank, e.name, e.score, e.penalties));
    }

    println!("{}", lines.join("\n"));
}
