pub struct Tag<'a> {
    pub name: &'a str,
    pub active: bool,
}

pub fn collect_active_names(tags: &[Tag<'_>]) -> Vec<String> {
    tags.iter()
        .filter(|tag| tag.active)
        .map(|tag| tag.name.trim())
        .map(|name| name.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{collect_active_names, Tag};

    #[test]
    fn keeps_only_active_non_empty_trimmed_names() {
        let tags = [
            Tag {
                name: "  alpha  ",
                active: true,
            },
            Tag {
                name: "   ",
                active: true,
            },
            Tag {
                name: "beta",
                active: false,
            },
            Tag {
                name: " gamma ",
                active: true,
            },
        ];

        let got = collect_active_names(&tags);
        assert_eq!(got, vec!["alpha", "gamma"]);
    }

    #[test]
    fn returns_empty_when_no_active_names_survive() {
        let tags = [
            Tag {
                name: "   ",
                active: true,
            },
            Tag {
                name: "delta",
                active: false,
            },
        ];

        let got = collect_active_names(&tags);
        assert!(got.is_empty());
    }
}
