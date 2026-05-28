struct Rep {
    name: &'static str,
    revenue: u32,
    units: u32,
}

fn main() {
    let mut reps = vec![
        Rep { name: "Bea", revenue: 120, units: 2 },
        Rep { name: "Ada", revenue: 120, units: 3 },
        Rep { name: "Dan", revenue: 95, units: 4 },
        Rep { name: "Cy", revenue: 95, units: 5 },
        Rep { name: "Eli", revenue: 120, units: 3 },
    ];

    reps.sort_by(|a, b| {
        b.revenue
            .cmp(&a.revenue)
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| b.units.cmp(&a.units))
    });

    let lines: Vec<String> = reps
        .iter()
        .enumerate()
        .map(|(i, r)| format!("{}. {} - ${} ({})", i + 1, r.name, r.revenue, r.units))
        .collect();

    println!("{}", lines.join("\n"));
}
