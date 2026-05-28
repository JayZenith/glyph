use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Record {
    pub category: &'static str,
    pub amount: i32,
    pub completed: bool,
}

pub fn build_report(records: &[Record]) -> String {
    let mut totals: BTreeMap<&str, (i32, usize)> = BTreeMap::new();

    for r in records {
        if !r.completed {
            continue;
        }
        let entry = totals.entry(r.category).or_insert((0, 0));
        entry.0 += r.amount;
        entry.1 += 1;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let grand_total: i32 = rows.iter().map(|(_, (sum, _))| *sum).sum();
    let mut out = format!("grand_total={grand_total}\n");
    for (category, (sum, count)) in rows {
        out.push_str(&format!("{category}: total={sum}, count={count}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_incomplete_and_non_positive_and_sorts_by_total_desc() {
        let records = [
            Record { category: "ops", amount: 50, completed: true },
            Record { category: "sales", amount: 70, completed: true },
            Record { category: "ops", amount: 25, completed: true },
            Record { category: "ops", amount: -5, completed: true },
            Record { category: "sales", amount: 0, completed: true },
            Record { category: "sales", amount: 100, completed: false },
            Record { category: "rnd", amount: 60, completed: true },
        ];

        let expected = concat!(
            "grand_total=205 categories=3\n",
            "ops: total=75, count=2\n",
            "sales: total=70, count=1\n",
            "rnd: total=60, count=1\n"
        );

        assert_eq!(build_report(&records), expected);
    }

    #[test]
    fn tie_on_total_uses_category_name() {
        let records = [
            Record { category: "zeta", amount: 40, completed: true },
            Record { category: "alpha", amount: 40, completed: true },
            Record { category: "alpha", amount: -3, completed: true },
            Record { category: "zeta", amount: 0, completed: true },
        ];

        let expected = concat!(
            "grand_total=80 categories=2\n",
            "alpha: total=40, count=1\n",
            "zeta: total=40, count=1\n"
        );

        assert_eq!(build_report(&records), expected);
    }
}
