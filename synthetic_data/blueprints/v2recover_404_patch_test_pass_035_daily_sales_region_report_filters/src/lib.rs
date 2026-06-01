use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale<'a> {
    pub region: &'a str,
    pub amount_cents: i32,
    pub refunded: bool,
}

pub fn build_region_report(sales: &[Sale<'_>]) -> String {
    let mut totals: BTreeMap<&str, (usize, i32)> = BTreeMap::new();

    for sale in sales {
        let entry = totals.entry(sale.region).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += sale.amount_cents;
    }

    let mut lines = Vec::new();
    for (region, (count, total)) in totals {
        if count == 0 {
            continue;
        }
        lines.push(format!("{}: {} sales, ${:.2}", region, count, total as f64 / 100.0));
    }

    if lines.is_empty() {
        "no sales".to_string()
    } else {
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_non_refunded_sales_by_region_and_sorts_by_region() {
        let sales = [
            Sale { region: "west", amount_cents: 2500, refunded: false },
            Sale { region: "east", amount_cents: 1200, refunded: false },
            Sale { region: "west", amount_cents: 500, refunded: true },
            Sale { region: "east", amount_cents: 800, refunded: false },
            Sale { region: "north", amount_cents: 0, refunded: false },
        ];

        let report = build_region_report(&sales);
        assert_eq!(report, "east: 2 sales, $20.00\nnorth: 1 sales, $0.00\nwest: 1 sales, $25.00");
    }

    #[test]
    fn returns_no_sales_when_all_entries_are_refunded() {
        let sales = [
            Sale { region: "west", amount_cents: 1000, refunded: true },
            Sale { region: "east", amount_cents: 2000, refunded: true },
        ];

        assert_eq!(build_region_report(&sales), "no sales");
    }

    #[test]
    fn keeps_negative_adjustments_for_non_refunded_sales() {
        let sales = [
            Sale { region: "central", amount_cents: 1500, refunded: false },
            Sale { region: "central", amount_cents: -200, refunded: false },
        ];

        assert_eq!(build_region_report(&sales), "central: 2 sales, $13.00");
    }
}
