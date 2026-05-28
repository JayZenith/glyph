use std::collections::BTreeMap;

struct Txn {
    month: &'static str,
    region: &'static str,
    amount: i32,
    kind: &'static str,
}

#[derive(Default)]
struct Summary {
    orders: usize,
    gross: i32,
    refunds: i32,
}

fn build_report(rows: &[Txn]) -> String {
    let mut grouped: BTreeMap<(&str, &str), Summary> = BTreeMap::new();

    for row in rows {
        let entry = grouped.entry((row.month, row.region)).or_default();
        if row.kind == "order" {
            entry.orders += 1;
            entry.gross += row.amount;
        } else {
            entry.refunds += row.amount;
            entry.orders += 1;
        }
    }

    let mut lines = Vec::new();
    for ((month, region), s) in grouped {
        let net = s.gross - s.refunds;
        lines.push(format!(
            "{} | {} | orders={} | gross={} | refunds={} | net={}",
            month, region, s.orders, s.gross, s.refunds, net
        ));
    }
    lines.join("\n")
}

fn main() {
    let rows = [
        Txn { month: "2024-01", region: "EU", amount: 120, kind: "order" },
        Txn { month: "2024-01", region: "EU", amount: 80, kind: "order" },
        Txn { month: "2024-01", region: "EU", amount: 50, kind: "refund" },
        Txn { month: "2024-01", region: "APAC", amount: 90, kind: "order" },
        Txn { month: "2024-01", region: "APAC", amount: 60, kind: "order" },
        Txn { month: "2024-01", region: "APAC", amount: 20, kind: "refund" },
        Txn { month: "2024-02", region: "EU", amount: 80, kind: "order" },
        Txn { month: "2024-02", region: "EU", amount: 10, kind: "refund" },
        Txn { month: "2024-02", region: "APAC", amount: 70, kind: "order" },
    ];

    println!("{}", build_report(&rows));
}
