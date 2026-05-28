use std::collections::BTreeMap;

fn build_report(records: &[(&str, i32)]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();

    for (dept, amount) in records {
        *totals.entry(*dept).or_insert(0) += *amount;
    }

    let mut rows: Vec<(&str, i32)> = totals.into_iter().collect();
    rows.sort_by_key(|(dept, total)| (*total, *dept));

    let grand_total: i32 = rows.iter().map(|(_, total)| *total).sum();

    let mut out = String::from("SALES REPORT\n");
    for (dept, total) in rows {
        out.push_str(&format!("{}: {}\n", dept, total));
    }
    out.push_str(&format!("GRAND TOTAL: {}", grand_total));
    out
}

fn main() {
    let records = [
        ("Books", 12),
        ("Games", 7),
        ("Games", -3),
        ("Books", 5),
        ("Garden", 10),
        ("Books", -8),
        ("Toys", 0),
        ("Games", 10),
        ("Toys", -4),
    ];

    println!("{}", build_report(&records));
}
