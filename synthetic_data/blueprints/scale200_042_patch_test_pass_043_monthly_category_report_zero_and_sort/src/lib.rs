use std::collections::BTreeMap;

pub fn sales_report(entries: &[(&str, i32)]) -> String {
    let mut totals = BTreeMap::new();

    for &(category, amount) in entries {
        *totals.entry(category).or_insert(0) += amount;
    }

    let mut rows: Vec<_> = totals.into_iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::new();
    for (idx, (category, total)) in rows.iter().enumerate() {
        if idx > 0 {
            out.push('\n');
        }
        out.push_str(category);
        out.push_str(": ");
        out.push_str(&total.to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::sales_report;

    #[test]
    fn groups_positive_sales_and_orders_by_total_then_name() {
        let entries = [
            ("books", 10),
            ("games", 5),
            ("books", 2),
            ("garden", 7),
            ("games", 7),
            ("office", 0),
            ("returns", -3),
        ];

        assert_eq!(sales_report(&entries), "books: 12\ngames: 12\ngarden: 7");
    }

    #[test]
    fn returns_no_sales_when_all_entries_are_non_positive() {
        let entries = [("books", 0), ("games", -2), ("garden", -1)];
        assert_eq!(sales_report(&entries), "no sales");
    }
}
