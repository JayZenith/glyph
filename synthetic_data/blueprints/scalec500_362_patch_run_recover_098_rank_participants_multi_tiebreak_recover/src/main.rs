struct Entry {
    name: &'static str,
    solved: u32,
    penalty: u32,
    last_ac: u32,
}

fn main() {
    let mut entries = vec![
        Entry { name: "Ada", solved: 5, penalty: 520, last_ac: 120 },
        Entry { name: "Bob", solved: 5, penalty: 520, last_ac: 110 },
        Entry { name: "Cara", solved: 5, penalty: 520, last_ac: 110 },
        Entry { name: "Dan", solved: 4, penalty: 410, last_ac: 80 },
        Entry { name: "Eve", solved: 5, penalty: 640, last_ac: 95 },
    ];

    entries.sort_by(|a, b| {
        b.solved
            .cmp(&a.solved)
            .then(b.penalty.cmp(&a.penalty))
            .then(b.last_ac.cmp(&a.last_ac))
    });

    for (idx, e) in entries.iter().enumerate() {
        println!(
            "{}. {} | solved={} penalty={} last={}",
            idx + 1,
            e.name,
            e.solved,
            e.penalty,
            e.last_ac
        );
    }
}
