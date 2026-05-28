use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub region: &'static str,
    pub amount: u32,
    pub refunded: bool,
}

pub fn net_sales_by_region(sales: &[Sale]) -> Vec<(String, u32)> {
    let mut totals: BTreeMap<&str, u32> = BTreeMap::new();

    for sale in sales {
        if !sale.refunded {
            *totals.entry(sale.region).or_insert(0) += sale.amount;
        }
    }

    totals
        .into_iter()
        .map(|(region, total)| (region.to_string(), total))
        .collect()
}

pub fn report_lines(sales: &[Sale]) -> Vec<String> {
    net_sales_by_region(sales)
        .into_iter()
        .map(|(region, total)| format!("{region}: {total}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_non_refunded_sales_by_region() {
        let sales = [
            Sale {
                region: "west",
                amount: 40,
                refunded: false,
            },
            Sale {
                region: "east",
                amount: 15,
                refunded: false,
            },
            Sale {
                region: "west",
                amount: 10,
                refunded: false,
            },
            Sale {
                region: "east",
                amount: 5,
                refunded: true,
            },
        ];

        let totals = net_sales_by_region(&sales);

        assert_eq!(
            totals,
            vec![("east".to_string(), 15), ("west".to_string(), 50)]
        );
    }

    #[test]
    fn report_lines_are_sorted_by_region_name() {
        let sales = [
            Sale {
                region: "south",
                amount: 7,
                refunded: false,
            },
            Sale {
                region: "north",
                amount: 3,
                refunded: false,
            },
            Sale {
                region: "north",
                amount: 2,
                refunded: true,
            },
        ];

        assert_eq!(
            report_lines(&sales),
            vec!["north: 3".to_string(), "south: 7".to_string()]
        );
    }

    #[test]
    fn empty_when_all_sales_are_refunded() {
        let sales = [
            Sale {
                region: "central",
                amount: 9,
                refunded: true,
            },
            Sale {
                region: "central",
                amount: 4,
                refunded: true,
            },
        ];

        assert!(net_sales_by_region(&sales).is_empty());
        assert!(report_lines(&sales).is_empty());
    }
}
