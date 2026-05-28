use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub category: &'static str,
    pub amount_cents: i32,
    pub refunded: bool,
}

pub fn summarize(sales: &[Sale]) -> String {
    let mut totals: BTreeMap<&str, (i32, usize)> = BTreeMap::new();

    for sale in sales {
        let entry = totals.entry(sale.category).or_insert((0, 0));
        entry.0 += sale.amount_cents;
        entry.1 += 1;
    }

    let mut lines = Vec::new();
    for (category, (total, count)) in totals {
        let avg = total / count as i32;
        lines.push(format!("{}:{}:{}", category, total, avg));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_non_refunded_sales_and_sorts_by_total_desc_then_name() {
        let sales = vec![
            Sale { category: "books", amount_cents: 1200, refunded: false },
            Sale { category: "games", amount_cents: 5000, refunded: false },
            Sale { category: "books", amount_cents: 800, refunded: false },
            Sale { category: "games", amount_cents: 500, refunded: true },
            Sale { category: "garden", amount_cents: 2000, refunded: false },
        ];

        assert_eq!(
            summarize(&sales),
            "games:5000:5000\nbooks:2000:1000\ngarden:2000:2000"
        );
    }

    #[test]
    fn omits_categories_with_zero_or_negative_total_after_refunds() {
        let sales = vec![
            Sale { category: "toys", amount_cents: 1000, refunded: false },
            Sale { category: "toys", amount_cents: -1000, refunded: false },
            Sale { category: "music", amount_cents: 3000, refunded: false },
            Sale { category: "music", amount_cents: -500, refunded: false },
            Sale { category: "office", amount_cents: -200, refunded: false },
        ];

        assert_eq!(summarize(&sales), "music:2500:1250");
    }
}
