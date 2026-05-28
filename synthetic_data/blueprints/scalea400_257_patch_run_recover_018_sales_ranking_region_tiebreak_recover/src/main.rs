use std::cmp::Reverse;

#[derive(Clone, Debug)]
struct Rep {
    name: &'static str,
    revenue: u32,
    returns: u32,
}

fn main() {
    let reps = vec![
        Rep { name: "Ava", revenue: 90, returns: 1 },
        Rep { name: "Mia", revenue: 120, returns: 0 },
        Rep { name: "Zoe", revenue: 120, returns: 3 },
        Rep { name: "Eli", revenue: 70, returns: 1 },
        Rep { name: "Ava", revenue: 60, returns: 1 },
        Rep { name: "Eli", revenue: 80, returns: 0 },
    ];

    let mut ranked = reps.clone();
    ranked.sort_by_key(|r| (Reverse(r.revenue), r.name, r.returns));

    let lines: Vec<String> = ranked
        .into_iter()
        .take(4)
        .enumerate()
        .map(|(i, r)| format!("{}. {} | revenue={} | returns={}", i + 1, r.name, r.revenue, r.returns))
        .collect();

    print!("FINAL: {}", lines.join("\n"));
}
