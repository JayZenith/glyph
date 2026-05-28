use std::collections::BTreeMap;

#[derive(Clone, Debug)]
struct Row {
    region: &'static str,
    sales: i32,
    returns: i32,
}

fn rows() -> Vec<Row> {
    vec![
        Row { region: "North", sales: 120, returns: 3 },
        Row { region: "South", sales: 90, returns: 1 },
        Row { region: "East", sales: 150, returns: 4 },
        Row { region: "West", sales: 110, returns: 2 },
        Row { region: "South", sales: 85, returns: 1 },
        Row { region: "North", sales: 70, returns: 2 },
        Row { region: "Central", sales: 120, returns: 1 },
        Row { region: "West", sales: 80, returns: 3 },
        Row { region: "East", sales: 40, returns: 0 },
    ]
}

fn main() {
    let mut totals: BTreeMap<&'static str, (i32, i32)> = BTreeMap::new();
    for row in rows() {
        totals.insert(row.region, (row.sales, row.returns));
    }

    let mut board: Vec<(&str, i32, i32)> = totals
        .into_iter()
        .map(|(region, (sales, returns))| (region, sales, returns))
        .collect();

    board.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then(a.0.cmp(b.0))
            .then(a.2.cmp(&b.2))
    });

    for (idx, (region, sales, returns)) in board.iter().enumerate() {
        println!("{}. {} | sales={} | returns={}", idx + 1, region, sales, returns);
    }
}
