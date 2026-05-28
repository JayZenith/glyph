#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: &'static str,
    pub score: u32,
    pub penalty: u32,
}

pub fn leaderboard(mut entries: Vec<Entry>) -> Vec<String> {
    entries.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.name.cmp(b.name))
            .then_with(|| a.penalty.cmp(&b.penalty))
    });

    entries
        .into_iter()
        .map(|e| format!("{}:{}:{}", e.name, e.score, e.penalty))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_score_then_penalty_then_name() {
        let entries = vec![
            Entry { name: "zoe", score: 10, penalty: 3 },
            Entry { name: "amy", score: 12, penalty: 5 },
            Entry { name: "bob", score: 12, penalty: 2 },
            Entry { name: "cara", score: 12, penalty: 2 },
            Entry { name: "dan", score: 10, penalty: 1 },
        ];

        let got = leaderboard(entries);
        let want = vec![
            "bob:12:2",
            "cara:12:2",
            "amy:12:5",
            "dan:10:1",
            "zoe:10:3",
        ];

        assert_eq!(got, want);
    }

    #[test]
    fn keeps_alphabetical_order_only_when_other_fields_tie() {
        let entries = vec![
            Entry { name: "mike", score: 7, penalty: 4 },
            Entry { name: "anna", score: 7, penalty: 1 },
            Entry { name: "zara", score: 7, penalty: 1 },
        ];

        let got = leaderboard(entries);
        let want = vec!["anna:7:1", "zara:7:1", "mike:7:4"];

        assert_eq!(got, want);
    }
}
