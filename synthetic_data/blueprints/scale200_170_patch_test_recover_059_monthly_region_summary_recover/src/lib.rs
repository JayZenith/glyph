use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order<'a> {
    pub region: &'a str,
    pub amount_cents: u32,
    pub shipped: bool,
    pub refunded: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RegionLine {
    pub region: String,
    pub shipped_orders: usize,
    pub revenue_cents: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Summary {
    pub lines: Vec<RegionLine>,
    pub total_shipped_orders: usize,
    pub total_revenue_cents: u32,
    pub top_region: Option<String>,
}

pub fn summarize(orders: &[Order<'_>]) -> Summary {
    let mut by_region: BTreeMap<&str, (usize, u32)> = BTreeMap::new();
    let mut total_shipped_orders = 0usize;
    let mut total_revenue_cents = 0u32;

    for order in orders {
        if !order.shipped {
            continue;
        }

        let entry = by_region.entry(order.region).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += order.amount_cents;
        total_shipped_orders += 1;
        total_revenue_cents += order.amount_cents;
    }

    let lines: Vec<RegionLine> = by_region
        .into_iter()
        .map(|(region, (shipped_orders, revenue_cents))| RegionLine {
            region: region.to_string(),
            shipped_orders,
            revenue_cents,
        })
        .collect();

    let top_region = lines
        .iter()
        .max_by_key(|line| line.revenue_cents)
        .map(|line| line.region.clone());

    Summary {
        lines,
        total_shipped_orders,
        total_revenue_cents,
        top_region,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excludes_refunded_orders_from_all_aggregates() {
        let orders = [
            Order { region: "west", amount_cents: 500, shipped: true, refunded: false },
            Order { region: "west", amount_cents: 700, shipped: true, refunded: true },
            Order { region: "east", amount_cents: 300, shipped: true, refunded: false },
            Order { region: "east", amount_cents: 200, shipped: false, refunded: false },
        ];

        let summary = summarize(&orders);

        assert_eq!(summary.total_shipped_orders, 2);
        assert_eq!(summary.total_revenue_cents, 800);
        assert_eq!(summary.lines, vec![
            RegionLine { region: "east".into(), shipped_orders: 1, revenue_cents: 300 },
            RegionLine { region: "west".into(), shipped_orders: 1, revenue_cents: 500 },
        ]);
    }

    #[test]
    fn top_region_uses_revenue_then_region_name_ascending_for_ties() {
        let orders = [
            Order { region: "south", amount_cents: 400, shipped: true, refunded: false },
            Order { region: "north", amount_cents: 400, shipped: true, refunded: false },
            Order { region: "south", amount_cents: 200, shipped: false, refunded: false },
            Order { region: "north", amount_cents: 100, shipped: true, refunded: true },
        ];

        let summary = summarize(&orders);

        assert_eq!(summary.top_region, Some("north".into()));
        assert_eq!(summary.lines, vec![
            RegionLine { region: "north".into(), shipped_orders: 1, revenue_cents: 400 },
            RegionLine { region: "south".into(), shipped_orders: 1, revenue_cents: 400 },
        ]);
    }

    #[test]
    fn empty_input_has_no_top_region() {
        let summary = summarize(&[]);
        assert_eq!(summary.total_shipped_orders, 0);
        assert_eq!(summary.total_revenue_cents, 0);
        assert!(summary.lines.is_empty());
        assert_eq!(summary.top_region, None);
    }
}
