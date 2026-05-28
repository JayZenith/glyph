use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Sale,
    Refund,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Txn {
    pub month: &'static str,
    pub kind: Kind,
    pub amount_cents: i64,
    pub completed: bool,
}

pub fn monthly_report(txns: &[Txn]) -> String {
    let mut totals: BTreeMap<&str, i64> = BTreeMap::new();

    for txn in txns {
        if !txn.completed {
            continue;
        }

        let delta = match txn.kind {
            Kind::Sale => txn.amount_cents,
            Kind::Refund => txn.amount_cents,
        };

        *totals.entry(txn.month).or_insert(0) += delta;
    }

    let mut lines = Vec::new();
    for (month, total) in totals {
        lines.push(format!("{}: {}", month, format_cents(total)));
    }
    lines.join("\n")
}

fn format_cents(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let abs = cents.abs();
    format!("{}${}.{:02}", sign, abs / 100, abs % 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_completed_transactions_by_month_with_refunds_subtracted() {
        let txns = [
            Txn { month: "2024-01", kind: Kind::Sale, amount_cents: 1200, completed: true },
            Txn { month: "2024-01", kind: Kind::Refund, amount_cents: 200, completed: true },
            Txn { month: "2024-02", kind: Kind::Sale, amount_cents: 500, completed: false },
            Txn { month: "2024-02", kind: Kind::Sale, amount_cents: 750, completed: true },
        ];

        assert_eq!(monthly_report(&txns), "2024-01: $10.00\n2024-02: $7.50");
    }

    #[test]
    fn keeps_negative_month_totals_and_sorted_output() {
        let txns = [
            Txn { month: "2024-03", kind: Kind::Refund, amount_cents: 900, completed: true },
            Txn { month: "2024-01", kind: Kind::Sale, amount_cents: 250, completed: true },
            Txn { month: "2024-03", kind: Kind::Sale, amount_cents: 100, completed: false },
        ];

        assert_eq!(monthly_report(&txns), "2024-01: $2.50\n2024-03: -$9.00");
    }
}
