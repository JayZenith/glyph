pub struct Tag<'a> {
    pub name: &'a str,
    pub active: bool,
}

pub fn collect_active_tags(tags: &[Tag<'_>]) -> Vec<String> {
    tags.iter()
        .filter(|tag| tag.active)
        .map(|tag| tag.name.trim())
        .map(str::to_uppercase)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_only_active_trimmed_uppercase_names() {
        let tags = [
            Tag { name: "  red ", active: true },
            Tag { name: "blue", active: false },
            Tag { name: " green", active: true },
        ];

        assert_eq!(collect_active_tags(&tags), vec!["RED", "GREEN"]);
    }

    #[test]
    fn skips_active_entries_whose_names_are_empty_after_trim() {
        let tags = [
            Tag { name: "   ", active: true },
            Tag { name: " apple ", active: true },
            Tag { name: "", active: true },
        ];

        assert_eq!(collect_active_tags(&tags), vec!["APPLE"]);
    }
}
