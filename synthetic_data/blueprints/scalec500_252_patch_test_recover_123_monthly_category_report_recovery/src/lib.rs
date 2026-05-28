use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Record<'a> {
    pub month: &'a str,
    pub category: &'a str,
    pub amount_cents: i64,
    pub active: bool,
}

pub fn render_report(records: &[Record<'_>]) -> String {
    let mut months: BTreeMap<&str, BTreeMap<&str, i64>> = BTreeMap::new();

    for r in records {
        months
            .entry(r.month)
            .or_default()
            .entry(r.category)
            .and_modify(|v| *v += r.amount_cents)
            .or_insert(r.amount_cents);
    }

    let mut out = Vec::new();
    for (month, cats) in months {
        out.push(format!("[{month}]"));
        let mut entries: Vec<_> = cats.into_iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        for (cat, total) in entries {
            out.push(format!("{cat}: {total}"));
        }
    }

    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{render_report, Record};

    #[test]
    fn groups_filters_and_sorts_report() {
        let rows = vec![
            Record { month: "2024-02", category: "ops", amount_cents: 500, active: true },
            Record { month: "2024-01", category: "food", amount_cents: 250, active: true },
            Record { month: "2024-02", category: "travel", amount_cents: 500, active: true },
            Record { month: "2024-01", category: "food", amount_cents: 50, active: true },
            Record { month: "2024-01", category: "misc", amount_cents: -10, active: true },
            Record { month: "2024-01", category: "archived", amount_cents: 999, active: false },
            Record { month: "2024-13", category: "bad", amount_cents: 100, active: true },
            Record { month: "2024-02", category: "food", amount_cents: 100, active: true },
        ];

        let got = render_report(&rows);
        let expected = "[2024-01]\nfood: 300\nTOTAL: 300\n[2024-02]\nops: 500\ntravel: 500\nfood: 100\nTOTAL: 1100";
        assert_eq!(got, expected);
    }

    #[test]
    fn tie_breaks_categories_by_name_after_total() {
        let rows = vec![
            Record { month: "2024-03", category: "zeta", amount_cents: 200, active: true },
            Record { month: "2024-03", category: "alpha", amount_cents: 200, active: true },
            Record { month: "2024-03", category: "mid", amount_cents: 150, active: true },
        ];

        let got = render_report(&rows);
        let expected = "[2024-03]\nalpha: 200\nzeta: 200\nmid: 150\nTOTAL: 550";
        assert_eq!(got, expected);
    }
}
