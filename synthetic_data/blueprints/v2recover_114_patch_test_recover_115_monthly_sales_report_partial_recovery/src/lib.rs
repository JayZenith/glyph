use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub rep: &'static str,
    pub month: &'static str,
    pub amount: i32,
    pub refunded: bool,
}

pub fn build_report(sales: &[Sale]) -> String {
    let mut by_month: BTreeMap<&str, (i32, usize)> = BTreeMap::new();

    for sale in sales {
        let entry = by_month.entry(sale.month).or_insert((0, 0));
        entry.0 += sale.amount;
        entry.1 += 1;
    }

    let mut out = String::new();
    for (month, (total, count)) in by_month {
        out.push_str(&format!("{}: total={}, sales={}\n", month, total, count));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Sale> {
        vec![
            Sale { rep: "Ana", month: "2024-01", amount: 120, refunded: false },
            Sale { rep: "Ben", month: "2024-01", amount: 80, refunded: true },
            Sale { rep: "Ana", month: "2024-02", amount: 50, refunded: false },
            Sale { rep: "Cara", month: "2024-02", amount: 150, refunded: false },
            Sale { rep: "Ana", month: "2024-02", amount: 40, refunded: true },
            Sale { rep: "Ben", month: "2024-03", amount: 90, refunded: false },
        ]
    }

    #[test]
    fn monthly_totals_ignore_refunds_and_count_only_kept_sales() {
        let report = build_report(&sample());
        let expected = concat!(
            "2024-01: total=120, sales=1\n",
            "2024-02: total=200, sales=2\n",
            "2024-03: total=90, sales=1\n",
            "grand total=410\n"
        );
        assert_eq!(report, expected);
    }

    #[test]
    fn empty_input_reports_only_zero_grand_total() {
        assert_eq!(build_report(&[]), "grand total=0\n");
    }
}
