use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale<'a> {
    pub category: &'a str,
    pub quantity: i32,
    pub unit_price: i32,
}

pub fn sales_report(items: &[Sale<'_>]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();

    for item in items {
        let amount = item.quantity * item.unit_price;
        *totals.entry(item.category).or_insert(0) += amount;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    rows.into_iter()
        .map(|(category, total)| format!("{}:{}", category, total))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{sales_report, Sale};

    #[test]
    fn groups_and_sorts_by_total_desc_then_name() {
        let items = [
            Sale { category: "books", quantity: 2, unit_price: 15 },
            Sale { category: "games", quantity: 1, unit_price: 30 },
            Sale { category: "books", quantity: 1, unit_price: 10 },
            Sale { category: "tools", quantity: 3, unit_price: 5 },
        ];

        assert_eq!(sales_report(&items), "books:40\ngames:30\ntools:15");
    }

    #[test]
    fn skips_non_positive_quantities_and_zero_totals() {
        let items = [
            Sale { category: "books", quantity: 0, unit_price: 50 },
            Sale { category: "books", quantity: -2, unit_price: 50 },
            Sale { category: "games", quantity: 1, unit_price: 20 },
            Sale { category: "tools", quantity: 2, unit_price: 0 },
            Sale { category: "garden", quantity: 3, unit_price: 4 },
        ];

        assert_eq!(sales_report(&items), "games:20\ngarden:12");
    }

    #[test]
    fn ties_use_category_name_ascending() {
        let items = [
            Sale { category: "zeta", quantity: 2, unit_price: 5 },
            Sale { category: "alpha", quantity: 1, unit_price: 10 },
            Sale { category: "beta", quantity: 5, unit_price: 1 },
        ];

        assert_eq!(sales_report(&items), "alpha:10\nzeta:10\nbeta:5");
    }
}
