use std::collections::BTreeMap;

#[derive(Clone, Debug)]
struct Entry {
    name: &'static str,
    score: u32,
    tries: u32,
}

fn best_entries(items: &[Entry]) -> Vec<Entry> {
    let mut by_name: BTreeMap<&'static str, Entry> = BTreeMap::new();
    for item in items {
        match by_name.get(item.name) {
            Some(prev)
                if item.score > prev.score
                    || (item.score == prev.score && item.tries < prev.tries) =>
            {
                by_name.insert(item.name, item.clone());
            }
            None => {
                by_name.insert(item.name, item.clone());
            }
            _ => {}
        }
    }
    by_name.into_values().collect()
}

fn render(entries: &[Entry]) -> String {
    let mut rows = best_entries(entries);
    rows.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .then(a.tries.cmp(&b.tries))
            .then(a.name.cmp(b.name))
    });

    let mut out = Vec::new();
    let mut rank = 0usize;
    let mut prev_score = None;

    for (idx, row) in rows.iter().enumerate() {
        if prev_score != Some(row.score) {
            rank = idx + 1;
            prev_score = Some(row.score);
        }
        out.push(format!("{}. {} | {} | {} tries", rank, row.name, row.score, row.tries));
    }

    out.join("\n")
}

fn main() {
    let data = vec![
        Entry {
            name: "Ana",
            score: 90,
            tries: 2,
        },
        Entry {
            name: "Bo",
            score: 90,
            tries: 1,
        },
        Entry {
            name: "Cy",
            score: 88,
            tries: 2,
        },
        Entry {
            name: "Ana",
            score: 85,
            tries: 1,
        },
        Entry {
            name: "Dia",
            score: 88,
            tries: 1,
        },
        Entry {
            name: "Eli",
            score: 90,
            tries: 1,
        },
        Entry {
            name: "Fay",
            score: 70,
            tries: 3,
        },
    ];

    println!("{}", render(&data));
}
