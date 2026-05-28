#[derive(Clone, Copy)]
struct Seller {
    name: &'static str,
    units: u32,
    revenue: u32,
}

fn main() {
    let mut sellers = vec![
        Seller { name: "Ava", units: 7, revenue: 120 },
        Seller { name: "Bo", units: 7, revenue: 140 },
        Seller { name: "Cy", units: 7, revenue: 140 },
        Seller { name: "Dee", units: 6, revenue: 130 },
        Seller { name: "Eli", units: 6, revenue: 150 },
    ];

    sellers.sort_by(|a, b| {
        b.units
            .cmp(&a.units)
            .then_with(|| a.name.cmp(b.name))
    });

    let mut out = Vec::new();
    let mut rank = 0;
    let mut prev_units = None;

    for s in sellers {
        if prev_units != Some(s.units) {
            rank += 1;
            prev_units = Some(s.units);
        }
        out.push(format!("{}. {} | {} | {}", rank, s.name, s.units, s.revenue));
    }

    print!("{}", out.join("\n"));
}
