#[derive(Clone, Debug)]
pub struct Item {
    pub category: &'static str,
    pub amount: i32,
    pub active: bool,
}

pub fn summarize(items: &[Item]) -> Vec<String> {
    let mut totals: Vec<(&str, i32, usize)> = Vec::new();

    for item in items {
        let mut found = false;
        for entry in &mut totals {
            if entry.0 == item.category {
                entry.1 += item.amount;
                entry.2 += 1;
                found = true;
                break;
            }
        }
        if !found {
            totals.push((item.category, item.amount, 1));
        }
    }

    totals.sort_by(|a, b| a.0.cmp(b.0));

    totals
        .into_iter()
        .map(|(category, total, count)| format!("{}:{}:{}", category, total, count))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{summarize, Item};

    #[test]
    fn groups_only_active_items() {
        let items = [
            Item { category: "tools", amount: 10, active: true },
            Item { category: "food", amount: 4, active: false },
            Item { category: "tools", amount: 5, active: true },
            Item { category: "food", amount: 8, active: true },
        ];

        assert_eq!(
            summarize(&items),
            vec!["tools:15:2", "food:8:1"]
        );
    }

    #[test]
    fn sorts_by_total_desc_then_category() {
        let items = [
            Item { category: "beta", amount: 6, active: true },
            Item { category: "alpha", amount: 6, active: true },
            Item { category: "zeta", amount: 2, active: true },
        ];

        assert_eq!(
            summarize(&items),
            vec!["alpha:6:1", "beta:6:1", "zeta:2:1"]
        );
    }
}
