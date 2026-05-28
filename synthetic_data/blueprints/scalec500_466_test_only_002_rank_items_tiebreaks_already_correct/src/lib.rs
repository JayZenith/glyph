#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: &'static str,
    pub solved: u32,
    pub penalty: u32,
}

pub fn leaderboard(entries: &[Entry]) -> Vec<String> {
    let mut items = entries.to_vec();
    items.sort_by(|a, b| {
        b.solved
            .cmp(&a.solved)
            .then_with(|| a.penalty.cmp(&b.penalty))
            .then_with(|| a.name.cmp(b.name))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(idx, e)| format!("{}. {} [{}|{}]", idx + 1, e.name, e.solved, e.penalty))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_solved_desc_then_penalty_asc_then_name() {
        let entries = vec![
            Entry { name: "zoe", solved: 4, penalty: 30 },
            Entry { name: "amy", solved: 5, penalty: 50 },
            Entry { name: "bob", solved: 5, penalty: 20 },
            Entry { name: "cyd", solved: 5, penalty: 20 },
            Entry { name: "dan", solved: 4, penalty: 10 },
        ];

        let lines = leaderboard(&entries);
        assert_eq!(
            lines,
            vec![
                "1. bob [5|20]",
                "2. cyd [5|20]",
                "3. amy [5|50]",
                "4. dan [4|10]",
                "5. zoe [4|30]",
            ]
        );
    }

    #[test]
    fn preserves_all_entries_even_when_metrics_match() {
        let entries = vec![
            Entry { name: "ivy", solved: 2, penalty: 9 },
            Entry { name: "abe", solved: 2, penalty: 9 },
            Entry { name: "max", solved: 1, penalty: 1 },
        ];

        let lines = leaderboard(&entries);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "1. abe [2|9]");
        assert_eq!(lines[1], "2. ivy [2|9]");
        assert_eq!(lines[2], "3. max [1|1]");
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let entries = Vec::new();
        let lines = leaderboard(&entries);
        assert!(lines.is_empty());
    }
}
