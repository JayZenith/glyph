use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Rep {
    name: &'static str,
    region: &'static str,
    revenue: u32,
    units: u32,
}

fn main() {
    let reps = vec![
        Rep { name: "Ava", region: "North", revenue: 220, units: 5 },
        Rep { name: "Zoe", region: "North", revenue: 220, units: 4 },
        Rep { name: "Eli", region: "South", revenue: 200, units: 10 },
        Rep { name: "Mia", region: "South", revenue: 200, units: 8 },
        Rep { name: "Ben", region: "East", revenue: 150, units: 9 },
        Rep { name: "Ian", region: "East", revenue: 150, units: 9 },
        Rep { name: "Gia", region: "West", revenue: 180, units: 8 },
        Rep { name: "Nia", region: "West", revenue: 180, units: 7 },
    ];

    let mut by_region: BTreeMap<&str, Vec<Rep>> = BTreeMap::new();
    for rep in reps {
        by_region.entry(rep.region).or_default().push(rep);
    }

    let mut lines = Vec::new();
    for (region, reps) in by_region {
        let top = reps
            .into_iter()
            .max_by(|a, b| {
                a.revenue
                    .cmp(&b.revenue)
                    .then(a.name.cmp(b.name))
                    .then(a.units.cmp(&b.units))
            })
            .unwrap();
        lines.push(format!(
            "{}: {} (revenue={}, units={})",
            region, top.name, top.revenue, top.units
        ));
    }

    println!("{}", lines.join("\n"));
}
