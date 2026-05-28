struct Rep {
    name: &'static str,
    deals: u32,
    region: &'static str,
}

fn main() {
    let mut reps = vec![
        Rep { name: "Zoe", deals: 12, region: "West" },
        Rep { name: "Ava", deals: 12, region: "East" },
        Rep { name: "Mia", deals: 12, region: "West" },
        Rep { name: "Ben", deals: 9, region: "East" },
        Rep { name: "Eli", deals: 12, region: "North" },
    ];

    reps.sort_by(|a, b| {
        b.deals
            .cmp(&a.deals)
            .then(a.name.cmp(&b.name))
            .then(a.region.cmp(&b.region))
    });

    for (i, rep) in reps.iter().enumerate() {
        println!("{}. {} | {} | {}", i + 1, rep.name, rep.deals, rep.region);
    }
}
