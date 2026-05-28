use std::collections::BTreeMap;

#[derive(Clone, Debug)]
struct Rep {
    region: &'static str,
    name: &'static str,
    sales: u32,
    returns: u32,
}

fn main() {
    let reps = vec![
        Rep { region: "North", name: "Ava", sales: 120, returns: 1 },
        Rep { region: "North", name: "Noah", sales: 120, returns: 3 },
        Rep { region: "North", name: "Mia", sales: 118, returns: 0 },
        Rep { region: "South", name: "Ben", sales: 95, returns: 2 },
        Rep { region: "South", name: "Cara", sales: 95, returns: 4 },
        Rep { region: "South", name: "Dax", sales: 90, returns: 1 },
        Rep { region: "East", name: "Zoe", sales: 120, returns: 2 },
        Rep { region: "East", name: "Eli", sales: 120, returns: 2 },
        Rep { region: "West", name: "Fay", sales: 105, returns: 0 },
        Rep { region: "West", name: "Gus", sales: 102, returns: 0 },
    ];

    let mut ranked = reps.clone();
    ranked.sort_by(|a, b| {
        b.sales
            .cmp(&a.sales)
            .then(a.name.cmp(&b.name))
            .then(a.returns.cmp(&b.returns))
    });

    let mut best_by_region: BTreeMap<&str, Rep> = BTreeMap::new();
    for rep in ranked {
        best_by_region.insert(rep.region, rep);
    }

    let lines: Vec<String> = best_by_region
        .into_iter()
        .map(|(region, rep)| format!("{}: {} (sales={}, returns={})", region, rep.name, rep.sales, rep.returns))
        .collect();

    print!("{}", lines.join("\n"));
}
