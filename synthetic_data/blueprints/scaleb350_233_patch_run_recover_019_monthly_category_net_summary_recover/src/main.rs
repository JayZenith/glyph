use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Entry {
    month: &'static str,
    kind: &'static str,
    status: &'static str,
    amount: i32,
}

fn report(entries: &[Entry]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();

    for e in entries {
        if e.status != "settled" {
            continue;
        }
        let signed = e.amount;
        *totals.entry(e.month).or_insert(0) += signed;
    }

    let mut lines = Vec::new();
    for (month, total) in totals {
        lines.push(format!("{} => {}", month, total));
    }

    lines.join("\n")
}

fn main() {
    let entries = [
        Entry { month: "2024-01", kind: "sale", status: "settled", amount: 120 },
        Entry { month: "2024-01", kind: "refund", status: "pending", amount: 30 },
        Entry { month: "2024-02", kind: "sale", status: "settled", amount: 50 },
        Entry { month: "2024-02", kind: "refund", status: "settled", amount: 50 },
        Entry { month: "2024-03", kind: "refund", status: "settled", amount: 15 },
        Entry { month: "2024-03", kind: "sale", status: "void", amount: 90 },
    ];

    println!("{}", report(&entries));
}
