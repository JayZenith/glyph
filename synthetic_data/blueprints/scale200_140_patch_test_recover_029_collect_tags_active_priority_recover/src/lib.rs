pub struct Item<'a> {
    pub name: &'a str,
    pub active: bool,
    pub priority: u8,
    pub tags: &'a [&'a str],
}

pub fn collect_tags(items: &[Item<'_>], min_priority: u8) -> Vec<String> {
    items
        .iter()
        .filter(|item| item.priority >= min_priority)
        .flat_map(|item| item.tags.iter().copied())
        .filter(|tag| !tag.is_empty())
        .map(|tag| tag.to_ascii_lowercase())
        .fold(Vec::new(), |mut acc, tag| {
            if !acc.contains(&tag) {
                acc.push(tag);
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_active_items_contribute_tags() {
        let items = [
            Item {
                name: "alpha",
                active: true,
                priority: 2,
                tags: &["Core", "fast"],
            },
            Item {
                name: "beta",
                active: false,
                priority: 5,
                tags: &["Hidden", "fast"],
            },
            Item {
                name: "gamma",
                active: true,
                priority: 1,
                tags: &["slow"],
            },
        ];

        assert_eq!(collect_tags(&items, 2), vec!["core", "fast"]);
    }

    #[test]
    fn trims_tags_and_skips_blank_entries() {
        let items = [
            Item {
                name: "delta",
                active: true,
                priority: 3,
                tags: &["  Ops ", "", " ops", "  ", "Build"],
            },
        ];

        assert_eq!(collect_tags(&items, 3), vec!["ops", "build"]);
    }

    #[test]
    fn keeps_first_seen_order_after_normalization() {
        let items = [
            Item {
                name: "one",
                active: true,
                priority: 4,
                tags: &[" API", "db"],
            },
            Item {
                name: "two",
                active: true,
                priority: 4,
                tags: &["api", " cache "],
            },
        ];

        assert_eq!(collect_tags(&items, 4), vec!["api", "db", "cache"]);
    }
}
