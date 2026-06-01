pub struct Tag<'a> {
    pub name: &'a str,
    pub enabled: bool,
}

pub fn collect_active_tag_names(tags: &[Tag<'_>]) -> Vec<String> {
    tags.iter()
        .filter(|tag| tag.enabled)
        .map(|tag| tag.name.to_ascii_lowercase())
        .filter(|name| name.len() >= 3)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{collect_active_tag_names, Tag};

    #[test]
    fn keeps_only_enabled_and_long_enough_names_in_order() {
        let tags = [
            Tag { name: "API", enabled: true },
            Tag { name: "ui", enabled: true },
            Tag { name: "Core", enabled: false },
            Tag { name: "Ops", enabled: true },
            Tag { name: "DB", enabled: true },
        ];

        let names = collect_active_tag_names(&tags);
        assert_eq!(names, vec!["api", "ops"]);
    }

    #[test]
    fn returns_empty_when_nothing_matches() {
        let tags = [
            Tag { name: "go", enabled: true },
            Tag { name: "Rust", enabled: false },
        ];

        let names = collect_active_tag_names(&tags);
        assert!(names.is_empty());
    }

    #[test]
    fn lowercases_mixed_case_names() {
        let tags = [
            Tag { name: "Net", enabled: true },
            Tag { name: "CLI", enabled: true },
        ];

        let names = collect_active_tag_names(&tags);
        assert_eq!(names, vec!["net", "cli"]);
    }
}
