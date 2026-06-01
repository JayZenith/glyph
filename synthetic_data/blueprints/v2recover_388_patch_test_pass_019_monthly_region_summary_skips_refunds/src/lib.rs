use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Shipped,
    Refunded,
    Canceled,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub region: &'static str,
    pub month: &'static str,
    pub amount_cents: u32,
    pub status: Status,
}

pub fn monthly_region_report(orders: &[Order]) -> String {
    let mut grouped: BTreeMap<(&str, &str), (u32, u32, u32)> = BTreeMap::new();

    for order in orders {
        let entry = grouped
            .entry((order.region, order.month))
            .or_insert((0, 0, 0));

        match order.status {
            Status::Shipped => {
                entry.0 += order.amount_cents;
                entry.1 += 1;
            }
            Status::Refunded => {
                entry.0 += order.amount_cents;
                entry.2 += 1;
            }
            Status::Canceled => {
                entry.1 += 1;
            }
        }
    }

    grouped
        .into_iter()
        .map(|((region, month), (total_cents, shipped_count, refunded_count))| {
            format!(
                "{} {} total=${:.2} shipped={} refunded={}",
                region,
                month,
                total_cents as f64 / 100.0,
                shipped_count,
                refunded_count
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{monthly_region_report, Order, Status};

    #[test]
    fn groups_by_region_and_month_with_refund_counts() {
        let orders = vec![
            Order {
                region: "east",
                month: "2024-01",
                amount_cents: 1200,
                status: Status::Shipped,
            },
            Order {
                region: "east",
                month: "2024-01",
                amount_cents: 800,
                status: Status::Refunded,
            },
            Order {
                region: "east",
                month: "2024-01",
                amount_cents: 400,
                status: Status::Canceled,
            },
            Order {
                region: "west",
                month: "2024-02",
                amount_cents: 500,
                status: Status::Shipped,
            },
            Order {
                region: "west",
                month: "2024-02",
                amount_cents: 250,
                status: Status::Shipped,
            },
            Order {
                region: "west",
                month: "2024-01",
                amount_cents: 300,
                status: Status::Refunded,
            },
        ];

        let report = monthly_region_report(&orders);
        assert_eq!(
            report,
            "east 2024-01 total=$12.00 shipped=1 refunded=1\nwest 2024-01 total=$0.00 shipped=0 refunded=1\nwest 2024-02 total=$7.50 shipped=2 refunded=0"
        );
    }

    #[test]
    fn canceled_only_groups_are_omitted() {
        let orders = vec![
            Order {
                region: "north",
                month: "2024-03",
                amount_cents: 999,
                status: Status::Canceled,
            },
            Order {
                region: "south",
                month: "2024-03",
                amount_cents: 2500,
                status: Status::Shipped,
            },
        ];

        let report = monthly_region_report(&orders);
        assert_eq!(report, "south 2024-03 total=$25.00 shipped=1 refunded=0");
    }
}
