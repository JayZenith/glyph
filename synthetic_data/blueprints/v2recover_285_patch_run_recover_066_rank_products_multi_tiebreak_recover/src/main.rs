use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Sale {
    name: &'static str,
    units: u32,
    revenue: u32,
}

fn build_rows(input: &[Sale]) -> Vec<Sale> {
    let mut totals: HashMap<&'static str, (u32, u32)> = HashMap::new();
    for s in input {
        totals.insert(s.name, (s.units, s.revenue));
    }

    let mut rows: Vec<Sale> = totals
        .into_iter()
        .map(|(name, (units, revenue))| Sale { name, units, revenue })
        .collect();

    rows.sort_by(|a, b| {
        a.revenue
            .cmp(&b.revenue)
            .then(a.units.cmp(&b.units))
            .then(b.name.cmp(a.name))
    });
    rows
}

fn main() {
    let sales = vec![
        Sale { name: "Nova", units: 6, revenue: 720 },
        Sale { name: "Apex", units: 4, revenue: 400 },
        Sale { name: "Bolt", units: 8, revenue: 1000 },
        Sale { name: "Nova", units: 4, revenue: 480 },
        Sale { name: "Ember", units: 8, revenue: 900 },
        Sale { name: "Apex", units: 7, revenue: 600 },
        Sale { name: "Cinder", units: 12, revenue: 600 },
    ];

    for (idx, row) in build_rows(&sales).into_iter().enumerate() {
        println!(
            "{}. {} | revenue={} | units= {}",
            idx + 1,
            row.name,
            row.revenue,
            row.units
        );
    }
}
