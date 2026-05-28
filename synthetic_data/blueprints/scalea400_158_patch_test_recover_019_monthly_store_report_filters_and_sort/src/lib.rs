use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Shipped,
    Returned,
    Cancelled,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub store: &'static str,
    pub amount_cents: i64,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoreReport {
    pub store: String,
    pub shipped: usize,
    pub returned: usize,
    pub net_cents: i64,
}

pub fn build_report(orders: &[Order]) -> Vec<StoreReport> {
    let mut totals: BTreeMap<&str, (usize, usize, i64)> = BTreeMap::new();

    for order in orders {
        let entry = totals.entry(order.store).or_insert((0, 0, 0));
        match order.status {
            Status::Shipped => {
                entry.0 += 1;
                entry.2 += order.amount_cents;
            }
            Status::Returned => {
                entry.1 += 1;
                entry.2 -= order.amount_cents;
            }
            Status::Cancelled => {
                entry.0 += 1;
            }
        }
    }

    let mut rows: Vec<StoreReport> = totals
        .into_iter()
        .map(|(store, (shipped, returned, net_cents))| StoreReport {
            store: store.to_string(),
            shipped,
            returned,
            net_cents,
        })
        .collect();

    rows.sort_by(|a, b| a.store.cmp(&b.store));
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_cancelled_and_zero_net_stores() {
        let orders = vec![
            Order { store: "north", amount_cents: 1000, status: Status::Shipped },
            Order { store: "north", amount_cents: 400, status: Status::Returned },
            Order { store: "north", amount_cents: 700, status: Status::Cancelled },
            Order { store: "south", amount_cents: 500, status: Status::Cancelled },
            Order { store: "east", amount_cents: 800, status: Status::Shipped },
            Order { store: "east", amount_cents: 800, status: Status::Returned },
        ];

        let report = build_report(&orders);

        assert_eq!(
            report,
            vec![StoreReport {
                store: "north".to_string(),
                shipped: 1,
                returned: 1,
                net_cents: 600,
            }]
        );
    }

    #[test]
    fn sorts_by_net_desc_then_store_name() {
        let orders = vec![
            Order { store: "delta", amount_cents: 500, status: Status::Shipped },
            Order { store: "alpha", amount_cents: 900, status: Status::Shipped },
            Order { store: "gamma", amount_cents: 900, status: Status::Shipped },
            Order { store: "beta", amount_cents: 300, status: Status::Shipped },
        ];

        let report = build_report(&orders);
        let stores: Vec<_> = report.iter().map(|r| r.store.as_str()).collect();
        let nets: Vec<_> = report.iter().map(|r| r.net_cents).collect();

        assert_eq!(stores, vec!["alpha", "gamma", "delta", "beta"]);
        assert_eq!(nets, vec![900, 900, 500, 300]);
    }
}
