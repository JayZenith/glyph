use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct Entry {
    month: &'static str,
    sales: i32,
    returns: i32,
}

fn build_report(entries: &[Entry]) -> String {
    let mut months: BTreeMap<&str, (i32, i32, i32)> = BTreeMap::new();

    for e in entries {
        let row = months.entry(e.month).or_insert((0, 0, 0));
        row.0 += 1;
        row.1 += e.sales;
        row.2 += e.returns;
    }

    let mut lines = Vec::new();
    let mut total_gross = 0;
    let mut total_returns = 0;

    for (month, (days, gross, returns)) in months {
        let net = gross - returns;
        total_gross += gross;
        total_returns += returns;
        lines.push(format!(
            "{month} | days={days} | gross={gross} | returns={returns} | net={net}"
        ));
    }

    lines.push(format!(
        "TOTAL | gross={} | returns={} | net={}",
        total_gross,
        total_returns,
        total_gross - total_returns
    ));

    lines.join("\n")
}

fn main() {
    let entries = [
        Entry {
            month: "2024-01",
            sales: 10,
            returns: 0,
        },
        Entry {
            month: "2024-01",
            sales: 5,
            returns: 2,
        },
        Entry {
            month: "2024-02",
            sales: 0,
            returns: 0,
        },
        Entry {
            month: "2024-02",
            sales: 0,
            returns: 0,
        },
        Entry {
            month: "2024-03",
            sales: 3,
            returns: 0,
        },
        Entry {
            month: "2024-03",
            sales: 4,
            returns: 1,
        },
    ];

    println!("{}", build_report(&entries));
}
