use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub store: &'static str,
    pub category: &'static str,
    pub units: u32,
}

pub fn category_report(sales: &[Sale], stores: &[&str]) -> Vec<String> {
    let mut totals: BTreeMap<&str, BTreeMap<&str, u32>> = BTreeMap::new();

    for sale in sales {
        let categories = totals.entry(sale.store).or_default();
        *categories.entry(sale.category).or_default() += sale.units;
    }

    let mut lines = Vec::new();
    for store in stores {
        if let Some(categories) = totals.get(store) {
            let mut parts = Vec::new();
            let mut grand_total = 0;
            for (category, units) in categories {
                if *units > 0 {
                    parts.push(format!("{}={}", category, units));
                    grand_total += units;
                }
            }
            if !parts.is_empty() {
                lines.push(format!("{} [{}] total={}", store, parts.join(", "), grand_total));
            }
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_requested_stores_in_input_order_with_zero_totals() {
        let sales = [
            Sale { store: "north", category: "books", units: 3 },
            Sale { store: "south", category: "games", units: 2 },
            Sale { store: "north", category: "games", units: 1 },
            Sale { store: "south", category: "games", units: 4 },
        ];

        let stores = ["south", "west", "north"];
        let report = category_report(&sales, &stores);

        assert_eq!(
            report,
            vec![
                "south [games=6] total=6",
                "west [no sales] total=0",
                "north [books=3, games=1] total=4",
            ]
        );
    }

    #[test]
    fn categories_are_aggregated_and_sorted_by_name() {
        let sales = [
            Sale { store: "central", category: "tools", units: 2 },
            Sale { store: "central", category: "appliances", units: 5 },
            Sale { store: "central", category: "tools", units: 3 },
            Sale { store: "central", category: "garden", units: 1 },
        ];

        let report = category_report(&sales, &["central"]);
        assert_eq!(report, vec!["central [appliances=5, garden=1, tools=5] total=11"]);
    }

    #[test]
    fn zero_unit_categories_are_shown_when_store_has_sales() {
        let sales = [
            Sale { store: "east", category: "books", units: 0 },
            Sale { store: "east", category: "music", units: 2 },
        ];

        let report = category_report(&sales, &["east"]);
        assert_eq!(report, vec!["east [books=0, music=2] total=2"]);
    }
}
