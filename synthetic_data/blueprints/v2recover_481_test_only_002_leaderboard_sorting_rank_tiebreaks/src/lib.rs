#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub score: u32,
    pub penalties: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut ordered = players.to_vec();
    ordered.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(a.penalties.cmp(&b.penalties))
            .then(a.name.cmp(b.name))
    });

    ordered
        .into_iter()
        .enumerate()
        .map(|(idx, p)| format!("{}. {} ({} pts, {} pen)", idx + 1, p.name, p.score, p.penalties))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_score_then_penalties_then_name() {
        let players = vec![
            Player { name: "Zoe", score: 15, penalties: 2 },
            Player { name: "Amy", score: 20, penalties: 5 },
            Player { name: "Ben", score: 20, penalties: 3 },
            Player { name: "Cal", score: 20, penalties: 3 },
        ];

        let got = leaderboard(&players);
        let expected = vec![
            "1. Ben (20 pts, 3 pen)",
            "2. Cal (20 pts, 3 pen)",
            "3. Amy (20 pts, 5 pen)",
            "4. Zoe (15 pts, 2 pen)",
        ];

        assert_eq!(got, expected);
    }

    #[test]
    fn duplicate_entries_are_not_removed_and_ranks_are_sequential() {
        let players = vec![
            Player { name: "Ivy", score: 10, penalties: 1 },
            Player { name: "Ivy", score: 10, penalties: 1 },
            Player { name: "Ada", score: 12, penalties: 4 },
        ];

        let got = leaderboard(&players);
        let expected = vec![
            "1. Ada (12 pts, 4 pen)",
            "2. Ivy (10 pts, 1 pen)",
            "3. Ivy (10 pts, 1 pen)",
        ];

        assert_eq!(got, expected);
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let got = leaderboard(&[]);
        assert!(got.is_empty());
    }
}
