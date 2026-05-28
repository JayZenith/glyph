#[derive(Debug)]
pub struct Item {
    pub name: &'static str,
    pub active: bool,
    pub score: Option<u32>,
    pub tags: &'static [&'static str],
}

pub fn select_names(items: &[Item], min_score: u32, required_tag: &str) -> Vec<&'static str> {
    items
        .iter()
        .filter(|item| item.active || item.tags.iter().any(|tag| *tag == required_tag))
        .filter_map(|item| item.score.map(|score| (item.name, score)))
        .filter(|(_, score)| *score > min_score)
        .map(|(name, _)| name)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{select_names, Item};

    #[test]
    fn keeps_only_active_tagged_and_meeting_minimum() {
        let items = [
            Item {
                name: "alpha",
                active: true,
                score: Some(7),
                tags: &["red", "hot"],
            },
            Item {
                name: "beta",
                active: true,
                score: Some(5),
                tags: &["blue"],
            },
            Item {
                name: "gamma",
                active: false,
                score: Some(9),
                tags: &["red"],
            },
            Item {
                name: "delta",
                active: true,
                score: None,
                tags: &["red"],
            },
            Item {
                name: "epsilon",
                active: true,
                score: Some(6),
                tags: &["red"],
            },
        ];

        assert_eq!(select_names(&items, 6, "red"), vec!["alpha", "epsilon"]);
    }

    #[test]
    fn includes_exact_minimum_and_preserves_order() {
        let items = [
            Item {
                name: "first",
                active: true,
                score: Some(3),
                tags: &["keep"],
            },
            Item {
                name: "second",
                active: true,
                score: Some(4),
                tags: &["keep", "extra"],
            },
            Item {
                name: "third",
                active: true,
                score: Some(10),
                tags: &["skip"],
            },
            Item {
                name: "fourth",
                active: true,
                score: Some(4),
                tags: &["keep"],
            },
        ];

        assert_eq!(select_names(&items, 4, "keep"), vec!["second", "fourth"]);
    }
}
