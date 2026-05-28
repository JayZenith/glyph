use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    pub month: &'static str,
    pub category: &'static str,
    pub amount: i32,
}

pub fn monthly_report(sales: &[Sale], month: &str) -> String {
    let mut totals: BTreeMap<&str, i32> = BTreeMap::new();

    for sale in sales {
        if sale.month == month && sale.amount != 0 {
            *totals.entry(sale.category).or_insert(0) += sale.amount;
        }
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let grand_total: i32 = rows.iter().map(|(_, amount)| *amount).sum();

    let mut out = format!("report for {}\n", month);
    for (category, amount) in rows {
        out.push_str(&format!("{}: {}\n", category, amount));
    }
    out.push_str(&format!("total: {}", grand_total));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_zero_only_categories_for_the_month() {
        let sales = [
            Sale { month: "2024-03", category: "hardware", amount: 0 },
            Sale { month: "2024-03", category: "books", amount: 5 },
            Sale { month: "2024-02", category: "hardware", amount: 9 },
        ];

        let report = monthly_report(&sales, "2024-03");
        let expected = "report for 2024-03\nbooks: 5\nhardware: 0\ntotal: 5";
        assert_eq!(report, expected);
    }

    #[test]
    fn sorts_by_total_desc_then_category_name() {
        let sales = [
            Sale { month: "2024-03", category: "garden", amount: 3 },
            Sale { month: "2024-03", category: "books", amount: 3 },
            Sale { month: "2024-03", category: "hardware", amount: 8 },
            Sale { month: "2024-03", category: "garden", amount: -1 },
            Sale { month: "2024-03", category: "books", amount: 0 },
        ];

        let report = monthly_report(&sales, "2024-03");
        let expected = "report for 2024-03\nhardware: 8\nbooks: 3\ngarden: 2\ntotal: 13";
        assert_eq!(report, expected);
    }
}
