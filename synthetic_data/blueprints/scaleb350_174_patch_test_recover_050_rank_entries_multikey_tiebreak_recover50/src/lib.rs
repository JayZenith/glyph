use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
    pub penalty: u32,
}

pub fn leaderboard(entries: &[Entry]) -> Vec<&'static str> {
    let mut items: Vec<&Entry> = entries.iter().collect();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.name.cmp(b.name))
    });
    items.into_iter().map(|e| e.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orders_by_score_then_wins_then_penalty_then_name() {
        let items = vec![
            Entry { name: "zeta", score: 20, wins: 3, penalty: 8 },
            Entry { name: "alpha", score: 20, wins: 4, penalty: 9 },
            Entry { name: "beta", score: 20, wins: 4, penalty: 3 },
            Entry { name: "gamma", score: 18, wins: 9, penalty: 1 },
        ];

        assert_eq!(leaderboard(&items), vec!["beta", "alpha", "zeta", "gamma"]);
    }

    #[test]
    fn final_name_tiebreak_is_case_insensitive_then_case_sensitive() {
        let items = vec![
            Entry { name: "Bravo", score: 10, wins: 2, penalty: 4 },
            Entry { name: "alpha", score: 10, wins: 2, penalty: 4 },
            Entry { name: "Alpha", score: 10, wins: 2, penalty: 4 },
            Entry { name: "bravo", score: 10, wins: 2, penalty: 4 },
        ];

        assert_eq!(leaderboard(&items), vec!["Alpha", "alpha", "Bravo", "bravo"]);
    }

    #[test]
    fn lower_penalty_only_matters_when_score_and_wins_tie() {
        let items = vec![
            Entry { name: "oak", score: 15, wins: 5, penalty: 20 },
            Entry { name: "elm", score: 15, wins: 6, penalty: 99 },
            Entry { name: "ash", score: 15, wins: 5, penalty: 1 },
        ];

        assert_eq!(leaderboard(&items), vec!["elm", "ash", "oak"]);
    }
}
