use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order<'a> {
    pub customer: &'a str,
    pub item_count: u32,
    pub unit_price_cents: u32,
    pub paid: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomerReport {
    pub customer: String,
    pub orders: u32,
    pub items: u32,
    pub revenue_cents: u32,
}

pub fn build_report(orders: &[Order<'_>]) -> Vec<CustomerReport> {
    let mut grouped: BTreeMap<&str, CustomerReport> = BTreeMap::new();

    for order in orders {
        let entry = grouped.entry(order.customer).or_insert_with(|| CustomerReport {
            customer: order.customer.to_string(),
            orders: 0,
            items: 0,
            revenue_cents: 0,
        });

        entry.orders += 1;
        entry.items += order.item_count;
        entry.revenue_cents += order.item_count * order.unit_price_cents;
    }

    let mut out: Vec<_> = grouped.into_values().collect();
    out.sort_by(|a, b| a.customer.cmp(&b.customer));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_paid_nonzero_orders_contribute() {
        let orders = vec![
            Order { customer: "Ava", item_count: 2, unit_price_cents: 500, paid: true },
            Order { customer: "Ava", item_count: 0, unit_price_cents: 900, paid: true },
            Order { customer: "Ava", item_count: 3, unit_price_cents: 700, paid: false },
            Order { customer: "Ben", item_count: 1, unit_price_cents: 1200, paid: true },
        ];

        let report = build_report(&orders);
        assert_eq!(
            report,
            vec![
                CustomerReport {
                    customer: "Ben".to_string(),
                    orders: 1,
                    items: 1,
                    revenue_cents: 1200,
                },
                CustomerReport {
                    customer: "Ava".to_string(),
                    orders: 1,
                    items: 2,
                    revenue_cents: 1000,
                },
            ]
        );
    }

    #[test]
    fn sorts_by_revenue_desc_then_customer_name() {
        let orders = vec![
            Order { customer: "Cara", item_count: 2, unit_price_cents: 500, paid: true },
            Order { customer: "Drew", item_count: 1, unit_price_cents: 1000, paid: true },
            Order { customer: "Cara", item_count: 1, unit_price_cents: 0, paid: true },
        ];

        let report = build_report(&orders);
        assert_eq!(report[0].customer, "Cara");
        assert_eq!(report[1].customer, "Drew");
        assert_eq!(report[0].revenue_cents, 1000);
        assert_eq!(report[1].revenue_cents, 1000);
    }

    #[test]
    fn zero_item_only_customers_are_omitted() {
        let orders = vec![
            Order { customer: "Nia", item_count: 0, unit_price_cents: 400, paid: true },
            Order { customer: "Omar", item_count: 2, unit_price_cents: 300, paid: true },
        ];

        let report = build_report(&orders);
        assert_eq!(
            report,
            vec![CustomerReport {
                customer: "Omar".to_string(),
                orders: 1,
                items: 2,
                revenue_cents: 600,
            }]
        );
    }
}
