use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Txn {
    dept: &'static str,
    amount: i32,
}

fn report(rows: &[Txn]) -> String {
    let mut totals: BTreeMap<&str, (i32, usize)> = BTreeMap::new();

    for row in rows {
        let entry = totals.entry(row.dept).or_insert((0, 0));
        entry.0 += row.amount;
        entry.1 += 1;
    }

    let mut items: Vec<_> = totals.into_iter().collect();
    items.sort_by(|a, b| a.0.cmp(b.0));

    items
        .into_iter()
        .map(|(dept, (total, count))| format!("{}: total={} count={}", dept, total, count))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let rows = [
        Txn { dept: "Sales", amount: 10 },
        Txn { dept: "HR", amount: 0 },
        Txn { dept: "Ops", amount: 15 },
        Txn { dept: "Sales", amount: 5 },
        Txn { dept: "HR", amount: 10 },
        Txn { dept: "Sales", amount: 0 },
    ];

    println!("{}", report(&rows));
}
