use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Order {
    pub region: &'static str,
    pub amount_cents: u32,
    pub completed: bool,
    pub priority: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionReport {
    pub region: String,
    pub completed_orders: usize,
    pub priority_completed_orders: usize,
    pub total_cents: u32,
}

pub fn build_region_report(orders: &[Order], regions: &[&str]) -> Vec<RegionReport> {
    let mut by_region: BTreeMap<&str, RegionReport> = BTreeMap::new();

    for order in orders {
        let entry = by_region.entry(order.region).or_insert_with(|| RegionReport {
            region: order.region.to_string(),
            completed_orders: 0,
            priority_completed_orders: 0,
            total_cents: 0,
        });

        entry.completed_orders += 1;
        entry.total_cents += order.amount_cents;
        if order.priority {
            entry.priority_completed_orders += 1;
        }
    }

    for region in regions {
        by_region.entry(region).or_insert_with(|| RegionReport {
            region: (*region).to_string(),
            completed_orders: 0,
            priority_completed_orders: 0,
            total_cents: 0,
        });
    }

    by_region
        .into_values()
        .filter(|row| row.completed_orders > 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_counts_only_completed_orders() {
        let orders = [
            Order { region: "east", amount_cents: 1200, completed: true, priority: true },
            Order { region: "east", amount_cents: 700, completed: false, priority: true },
            Order { region: "west", amount_cents: 400, completed: true, priority: false },
        ];

        let report = build_region_report(&orders, &["east", "west"]);

        assert_eq!(
            report,
            vec![
                RegionReport {
                    region: "east".to_string(),
                    completed_orders: 1,
                    priority_completed_orders: 1,
                    total_cents: 1200,
                },
                RegionReport {
                    region: "west".to_string(),
                    completed_orders: 1,
                    priority_completed_orders: 0,
                    total_cents: 400,
                },
            ]
        );
    }

    #[test]
    fn report_keeps_requested_regions_even_when_only_pending_orders_exist() {
        let orders = [
            Order { region: "north", amount_cents: 300, completed: false, priority: true },
            Order { region: "south", amount_cents: 500, completed: true, priority: false },
        ];

        let report = build_region_report(&orders, &["north", "south", "west"]);

        assert_eq!(
            report,
            vec![
                RegionReport {
                    region: "north".to_string(),
                    completed_orders: 0,
                    priority_completed_orders: 0,
                    total_cents: 0,
                },
                RegionReport {
                    region: "south".to_string(),
                    completed_orders: 1,
                    priority_completed_orders: 0,
                    total_cents: 500,
                },
                RegionReport {
                    region: "west".to_string(),
                    completed_orders: 0,
                    priority_completed_orders: 0,
                    total_cents: 0,
                },
            ]
        );
    }

    #[test]
    fn report_ignores_unrequested_regions_without_completed_orders() {
        let orders = [
            Order { region: "east", amount_cents: 200, completed: true, priority: false },
            Order { region: "ghost", amount_cents: 999, completed: false, priority: true },
        ];

        let report = build_region_report(&orders, &["east"]);

        assert_eq!(
            report,
            vec![RegionReport {
                region: "east".to_string(),
                completed_orders: 1,
                priority_completed_orders: 0,
                total_cents: 200,
            }]
        );
    }
}
