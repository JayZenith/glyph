#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub name: &'static str,
    pub enabled: bool,
    pub weight: u8,
}

pub fn collect_visible_tags(tags: &[Tag], min_weight: u8) -> Vec<&'static str> {
    tags.iter()
        .filter(|tag| tag.enabled)
        .filter_map(|tag| (tag.weight > min_weight).then_some(tag.name))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tags() -> Vec<Tag> {
        vec![
            Tag { name: "alpha", enabled: true, weight: 2 },
            Tag { name: "beta", enabled: false, weight: 9 },
            Tag { name: "gamma", enabled: true, weight: 5 },
            Tag { name: "delta", enabled: true, weight: 3 },
        ]
    }

    #[test]
    fn keeps_enabled_tags_at_threshold_or_higher() {
        let tags = sample_tags();
        assert_eq!(collect_visible_tags(&tags, 3), vec!["gamma", "delta"]);
    }

    #[test]
    fn excludes_disabled_even_if_heavy() {
        let tags = sample_tags();
        assert_eq!(collect_visible_tags(&tags, 8), Vec::<&'static str>::new());
    }

    #[test]
    fn preserves_input_order() {
        let tags = vec![
            Tag { name: "one", enabled: true, weight: 4 },
            Tag { name: "two", enabled: true, weight: 4 },
            Tag { name: "three", enabled: true, weight: 1 },
        ];
        assert_eq!(collect_visible_tags(&tags, 4), vec!["one", "two"]);
    }
}
