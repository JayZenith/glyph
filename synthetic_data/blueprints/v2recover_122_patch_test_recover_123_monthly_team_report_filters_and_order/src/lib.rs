#[derive(Clone, Debug)]
pub struct Entry {
    pub team: &'static str,
    pub amount: i32,
    pub refunded: bool,
}

pub fn build_report(entries: &[Entry]) -> String {
    let mut rows: Vec<(&str, i32, usize)> = Vec::new();

    for e in entries {
        if let Some((_, total, count)) = rows.iter_mut().find(|(team, _, _)| *team == e.team) {
            *total += e.amount;
            *count += 1;
        } else {
            rows.push((e.team, e.amount, 1));
        }
    }

    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::new();
    for (team, total, count) in rows {
        out.push_str(&format!("{}: total={}, entries={}\n", team, total, count));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Entry> {
        vec![
            Entry { team: "ops", amount: 30, refunded: false },
            Entry { team: "sales", amount: 20, refunded: false },
            Entry { team: "ops", amount: 10, refunded: true },
            Entry { team: "sales", amount: 5, refunded: false },
            Entry { team: "support", amount: 0, refunded: false },
            Entry { team: "ops", amount: 15, refunded: false },
        ]
    }

    #[test]
    fn excludes_refunds_and_zero_totals() {
        let got = build_report(&sample());
        let expected = "ops: total=45, entries=2\nsales: total=25, entries=2";
        assert_eq!(got, expected);
    }

    #[test]
    fn sorts_by_total_desc_then_team_name() {
        let entries = vec![
            Entry { team: "beta", amount: 10, refunded: false },
            Entry { team: "alpha", amount: 10, refunded: false },
            Entry { team: "zeta", amount: 3, refunded: false },
        ];
        let got = build_report(&entries);
        let expected = "alpha: total=10, entries=1\nbeta: total=10, entries=1\nzeta: total=3, entries=1";
        assert_eq!(got, expected);
    }
}
