#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item<'a> {
    pub name: &'a str,
    pub active: bool,
    pub priority: u8,
}

pub fn collect_tags(items: &[Item<'_>]) -> Vec<String> {
    items
        .iter()
        .filter(|item| item.active || item.priority >= 3)
        .map(|item| format!("{}:{}", item.name, item.priority))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_only_active_items_with_nonempty_names() {
        let items = [
            Item { name: "alpha", active: true, priority: 2 },
            Item { name: "", active: true, priority: 9 },
            Item { name: "beta", active: false, priority: 7 },
            Item { name: "gamma", active: true, priority: 0 },
        ];

        assert_eq!(
            collect_tags(&items),
            vec!["alpha:P2".to_string(), "gamma:P0".to_string()]
        );
    }

    #[test]
    fn preserves_input_order_for_matching_items() {
        let items = [
            Item { name: "first", active: true, priority: 1 },
            Item { name: "skip", active: false, priority: 8 },
            Item { name: "second", active: true, priority: 4 },
        ];

        assert_eq!(
            collect_tags(&items),
            vec!["first:P1".to_string(), "second:P4".to_string()]
        );
    }
}
