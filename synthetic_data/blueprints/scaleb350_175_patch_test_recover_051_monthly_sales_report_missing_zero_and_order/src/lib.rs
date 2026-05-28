use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale<'a> {
    pub category: &'a str,
    pub qty: i32,
}

pub fn render_report(sales: &[Sale<'_>], categories: &[&str]) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();

    for sale in sales {
        if sale.qty > 0 {
            *totals.entry(sale.category).or_insert(0) += sale.qty;
        }
    }

    let mut lines = Vec::new();
    for (category, total) in totals {
        if categories.contains(&category) {
            lines.push(format!("{}:{}", category, total));
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_non_positive_and_keeps_requested_order() {
        let sales = [
            Sale { category: "tools", qty: 3 },
            Sale { category: "books", qty: 5 },
            Sale { category: "tools", qty: -2 },
            Sale { category: "games", qty: 4 },
            Sale { category: "books", qty: 0 },
        ];

        let out = render_report(&sales, &["games", "books", "tools"]);
        assert_eq!(out, "games:4\nbooks:5\ntools:3");
    }

    #[test]
    fn includes_missing_requested_categories_as_zero() {
        let sales = [
            Sale { category: "books", qty: 2 },
            Sale { category: "books", qty: 3 },
            Sale { category: "tools", qty: -5 },
        ];

        let out = render_report(&sales, &["tools", "books", "garden"]);
        assert_eq!(out, "tools:0\nbooks:5\ngarden:0");
    }
}
