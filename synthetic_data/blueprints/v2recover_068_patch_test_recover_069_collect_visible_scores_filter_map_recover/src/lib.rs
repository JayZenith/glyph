pub struct Entry<'a> {
    pub name: &'a str,
    pub score: i32,
    pub visible: bool,
}

pub fn collect_labels(entries: &[Entry<'_>]) -> Vec<String> {
    entries
        .iter()
        .filter(|e| e.score > 0)
        .map(|e| format!("{}:{}", e.name, e.score + 1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{collect_labels, Entry};

    #[test]
    fn keeps_only_visible_nonempty_positive_entries_in_order() {
        let items = [
            Entry { name: "alpha", score: 2, visible: true },
            Entry { name: "", score: 5, visible: true },
            Entry { name: "beta", score: 0, visible: true },
            Entry { name: "gamma", score: 3, visible: false },
            Entry { name: "delta", score: 1, visible: true },
        ];

        assert_eq!(collect_labels(&items), vec!["alpha:2", "delta:1"]);
    }

    #[test]
    fn returns_empty_when_nothing_qualifies() {
        let items = [
            Entry { name: "", score: 4, visible: true },
            Entry { name: "hide", score: 7, visible: false },
            Entry { name: "neg", score: -2, visible: true },
        ];

        let out: Vec<String> = Vec::new();
        assert_eq!(collect_labels(&items), out);
    }
}
