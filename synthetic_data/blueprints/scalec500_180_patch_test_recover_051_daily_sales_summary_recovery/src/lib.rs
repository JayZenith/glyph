use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub category: &'static str,
    pub amount: i32,
    pub refunded: bool,
}

pub fn summarize_sales(sales: &[Sale]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();

    for sale in sales {
        let entry = totals.entry(sale.category).or_insert(0);
        if !sale.refunded {
            *entry += sale.amount;
        }
    }

    let mut lines = Vec::new();
    for (category, total) in totals {
        lines.push(format!("{}={}", category, total));
    }

    if lines.is_empty() {
        "no sales".to_string()
    } else {
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::{summarize_sales, Sale};

    #[test]
    fn groups_sorted_categories_with_net_totals() {
        let sales = [
            Sale { category: "books", amount: 30, refunded: false },
            Sale { category: "games", amount: 50, refunded: false },
            Sale { category: "books", amount: 10, refunded: true },
            Sale { category: "books", amount: 5, refunded: false },
        ];

        assert_eq!(summarize_sales(&sales), "books=25\ngames=50");
    }

    #[test]
    fn omits_categories_with_zero_or_negative_net_total() {
        let sales = [
            Sale { category: "books", amount: 10, refunded: false },
            Sale { category: "books", amount: 10, refunded: true },
            Sale { category: "games", amount: 20, refunded: false },
            Sale { category: "music", amount: 5, refunded: true },
        ];

        assert_eq!(summarize_sales(&sales), "games=20");
    }

    #[test]
    fn returns_no_sales_when_nothing_positive_remains() {
        let sales = [
            Sale { category: "books", amount: 7, refunded: true },
            Sale { category: "games", amount: 4, refunded: false },
            Sale { category: "games", amount: 4, refunded: true },
        ];

        assert_eq!(summarize_sales(&sales), "no sales");
    }
}
