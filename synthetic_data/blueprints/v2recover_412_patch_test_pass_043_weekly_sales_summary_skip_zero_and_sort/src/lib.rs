use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub team: &'static str,
    pub amount: i32,
}

pub fn summarize_sales(sales: &[Sale]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();
    let mut count = 0;

    for sale in sales {
        *totals.entry(sale.team).or_insert(0) += sale.amount;
        count += 1;
    }

    let mut lines = Vec::new();
    for (team, total) in totals {
        lines.push(format!("{team}: {total}"));
    }

    format!("teams={} entries={}\n{}", lines.len(), count, lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_positive_sales_and_orders_by_total_desc_then_name() {
        let sales = [
            Sale { team: "blue", amount: 5 },
            Sale { team: "red", amount: 7 },
            Sale { team: "blue", amount: 3 },
            Sale { team: "green", amount: 7 },
            Sale { team: "red", amount: -2 },
        ];

        let report = summarize_sales(&sales);
        assert_eq!(report, "teams=3 entries=5\nblue: 8\ngreen: 7\nred: 5");
    }

    #[test]
    fn omits_zero_total_teams_but_keeps_entry_count() {
        let sales = [
            Sale { team: "ops", amount: 4 },
            Sale { team: "ops", amount: -4 },
            Sale { team: "dev", amount: 2 },
            Sale { team: "dev", amount: 1 },
        ];

        let report = summarize_sales(&sales);
        assert_eq!(report, "teams=1 entries=4\ndev: 3");
    }

    #[test]
    fn empty_input_has_header_only() {
        let report = summarize_sales(&[]);
        assert_eq!(report, "teams=0 entries=0");
    }
}
