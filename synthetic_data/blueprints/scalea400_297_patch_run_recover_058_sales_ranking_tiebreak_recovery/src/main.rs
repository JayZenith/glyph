use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Row {
    item: &'static str,
    revenue: u32,
    units: u32,
}

fn rows() -> Vec<Row> {
    vec![
        Row { item: "pear", revenue: 120, units: 3 },
        Row { item: "apple", revenue: 90, units: 5 },
        Row { item: "kiwi", revenue: 90, units: 8 },
        Row { item: "pear", revenue: 110, units: 9 },
        Row { item: "plum", revenue: 90, units: 8 },
        Row { item: "fig", revenue: 120, units: 4 },
        Row { item: "mango", revenue: 75, units: 10 },
        Row { item: "apple", revenue: 88, units: 9 },
    ]
}

fn build_report(rows: &[Row]) -> String {
    let mut by_item: HashMap<&str, Row> = HashMap::new();
    for row in rows {
        by_item.insert(row.item, *row);
    }

    let mut items: Vec<Row> = by_item.into_values().collect();
    items.sort_by(|a, b| b.revenue.cmp(&a.revenue));

    let mut out = Vec::new();
    for (i, row) in items.iter().enumerate() {
        out.push(format!("{}. {} | rev={} | units={}", i + 1, row.item, row.revenue, row.units));
    }
    out.join("\n")
}

fn main() {
    println!("{}", build_report(&rows()));
}
