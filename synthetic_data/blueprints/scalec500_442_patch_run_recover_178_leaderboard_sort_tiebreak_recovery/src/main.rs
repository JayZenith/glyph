#[derive(Clone)]
struct Entry {
    name: &'static str,
    score: u32,
    penalty: u32,
}

fn main() {
    let mut entries = vec![
        Entry { name: "Zoe", score: 9, penalty: 0 },
        Entry { name: "Ava", score: 15, penalty: 2 },
        Entry { name: "Bea", score: 15, penalty: 1 },
        Entry { name: "Cy", score: 12, penalty: 3 },
        Entry { name: "Dan", score: 12, penalty: 0 },
        Entry { name: "Eli", score: 15, penalty: 1 },
    ];

    entries.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .reverse()
            .then(a.name.cmp(b.name))
    });

    let mut out = Vec::new();
    for (i, e) in entries.iter().enumerate() {
        out.push(format!("{}. {} {} {}", i + 1, e.name, e.score, e.penalty));
    }

    print!("{}", out.join("\n"));
}
