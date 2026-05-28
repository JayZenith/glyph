use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub team: &'static str,
    pub month: u8,
    pub amount: i32,
    pub refunded: bool,
}

pub fn build_report(sales: &[Sale]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();
    let mut active_months: BTreeMap<&str, usize> = BTreeMap::new();

    for sale in sales {
        if sale.refunded {
            continue;
        }
        *totals.entry(sale.team).or_insert(0) += sale.amount;
        *active_months.entry(sale.team).or_insert(0) += 1;
    }

    let mut out = String::new();
    for (team, total) in totals {
        let months = active_months.get(team).copied().unwrap_or(0);
        let avg = if months == 0 { 0 } else { total / months as i32 };
        out.push_str(&format!("{}: total={}, avg={}\n", team, total, avg));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_by_team_sorts_and_uses_distinct_months() {
        let sales = vec![
            Sale { team: "beta", month: 1, amount: 50, refunded: false },
            Sale { team: "alpha", month: 1, amount: 20, refunded: false },
            Sale { team: "beta", month: 1, amount: 30, refunded: false },
            Sale { team: "alpha", month: 2, amount: 25, refunded: false },
        ];

        let report = build_report(&sales);
        assert_eq!(report, "alpha: total=45, avg=22\nbeta: total=80, avg=80\n");
    }

    #[test]
    fn ignores_refunds_and_zero_total_teams() {
        let sales = vec![
            Sale { team: "ops", month: 3, amount: 10, refunded: true },
            Sale { team: "ops", month: 3, amount: 5, refunded: false },
            Sale { team: "ops", month: 4, amount: -5, refunded: false },
            Sale { team: "sales", month: 2, amount: 40, refunded: false },
            Sale { team: "sales", month: 2, amount: 10, refunded: true },
        ];

        let report = build_report(&sales);
        assert_eq!(report, "sales: total=40, avg=40\n");
    }
}
