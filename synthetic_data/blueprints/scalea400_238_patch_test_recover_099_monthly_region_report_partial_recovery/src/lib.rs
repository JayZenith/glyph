use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub month: &'static str,
    pub region: &'static str,
    pub amount_cents: i64,
    pub active: bool,
    pub kind: Kind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Sale,
    Refund,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Totals {
    gross_sales: i64,
    refunds: i64,
    active_rows: usize,
}

pub fn build_report(entries: &[Entry]) -> String {
    let mut grouped: BTreeMap<(&str, &str), Totals> = BTreeMap::new();

    for e in entries {
        let totals = grouped.entry((e.month, e.region)).or_default();
        if e.active {
            totals.active_rows += 1;
        }
        match e.kind {
            Kind::Sale => totals.gross_sales += e.amount_cents,
            Kind::Refund => totals.gross_sales -= e.amount_cents,
        }
    }

    let mut lines = Vec::new();
    for ((month, region), t) in grouped.into_iter().rev() {
        let net = t.gross_sales - t.refunds;
        lines.push(format!(
            "{month}|{region}|active={}|gross={}|refunds={}|net={}",
            t.active_rows, t.gross_sales, t.refunds, net
        ));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Entry> {
        vec![
            Entry { month: "2024-01", region: "east", amount_cents: 500, active: true, kind: Kind::Sale },
            Entry { month: "2024-01", region: "east", amount_cents: 120, active: true, kind: Kind::Refund },
            Entry { month: "2024-01", region: "east", amount_cents: 0, active: true, kind: Kind::Sale },
            Entry { month: "2024-01", region: "west", amount_cents: 300, active: false, kind: Kind::Sale },
            Entry { month: "2024-02", region: "east", amount_cents: 200, active: true, kind: Kind::Sale },
            Entry { month: "2024-02", region: "east", amount_cents: 50, active: false, kind: Kind::Refund },
            Entry { month: "2024-02", region: "north", amount_cents: 700, active: true, kind: Kind::Sale },
            Entry { month: "2024-02", region: "north", amount_cents: 100, active: true, kind: Kind::Refund },
        ]
    }

    #[test]
    fn report_groups_filters_and_orders() {
        let out = build_report(&sample());
        let expected = concat!(
            "2024-01|east|active=2|gross=500|refunds=120|net=380\n",
            "2024-02|east|active=1|gross=200|refunds=0|net=200\n",
            "2024-02|north|active=2|gross=700|refunds=100|net=600"
        );
        assert_eq!(out, expected);
    }

    #[test]
    fn empty_after_filtering_yields_empty_report() {
        let entries = vec![
            Entry { month: "2024-03", region: "south", amount_cents: 0, active: true, kind: Kind::Sale },
            Entry { month: "2024-03", region: "south", amount_cents: 20, active: false, kind: Kind::Refund },
        ];
        assert_eq!(build_report(&entries), "");
    }
}
