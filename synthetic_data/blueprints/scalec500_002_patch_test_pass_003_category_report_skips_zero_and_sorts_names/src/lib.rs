use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub category: &'static str,
    pub name: &'static str,
    pub units: u32,
}

pub fn category_report(items: &[Item]) -> String {
    let mut grouped: BTreeMap<&str, (u32, Vec<&str>)> = BTreeMap::new();

    for item in items {
        let entry = grouped.entry(item.category).or_insert((0, Vec::new()));
        entry.0 += item.units;
        entry.1.push(item.name);
    }

    let mut lines = Vec::new();
    for (category, (total, names)) in grouped {
        lines.push(format!("{}:{} [{}]", category, total, names.join(",")));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{category_report, Item};

    #[test]
    fn groups_by_category_sorts_names_and_skips_zero_totals() {
        let items = vec![
            Item { category: "fruit", name: "pear", units: 2 },
            Item { category: "fruit", name: "apple", units: 3 },
            Item { category: "tools", name: "hammer", units: 0 },
            Item { category: "tools", name: "wrench", units: 0 },
            Item { category: "books", name: "rust", units: 1 },
        ];

        let report = category_report(&items);
        assert_eq!(report, "books:1 [rust]\nfruit:5 [apple,pear]");
    }

    #[test]
    fn keeps_duplicate_names_when_multiple_items_exist() {
        let items = vec![
            Item { category: "office", name: "pen", units: 1 },
            Item { category: "office", name: "pen", units: 2 },
            Item { category: "office", name: "paper", units: 5 },
        ];

        let report = category_report(&items);
        assert_eq!(report, "office:8 [paper,pen,pen]");
    }
}
