use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub score: u32,
    pub solved: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by_key(|p| (Reverse(p.score), p.name, Reverse(p.solved)));

    items
        .into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} ({}/{})", i + 1, p.name, p.score, p.solved))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_score_then_solved_then_name() {
        let players = vec![
            Player { name: "zoe", score: 90, solved: 4 },
            Player { name: "amy", score: 100, solved: 3 },
            Player { name: "bob", score: 100, solved: 5 },
            Player { name: "cara", score: 90, solved: 6 },
        ];

        let got = leaderboard(&players);
        let want = vec![
            "1. bob (100/5)",
            "2. amy (100/3)",
            "3. cara (90/6)",
            "4. zoe (90/4)",
        ];

        assert_eq!(got, want);
    }

    #[test]
    fn uses_dense_ranks_for_ties() {
        let players = vec![
            Player { name: "amy", score: 100, solved: 5 },
            Player { name: "bob", score: 100, solved: 5 },
            Player { name: "cara", score: 95, solved: 7 },
            Player { name: "dan", score: 95, solved: 7 },
            Player { name: "eve", score: 80, solved: 9 },
        ];

        let got = leaderboard(&players);
        let want = vec![
            "1. amy (100/5)",
            "1. bob (100/5)",
            "2. cara (95/7)",
            "2. dan (95/7)",
            "3. eve (80/9)",
        ];

        assert_eq!(got, want);
    }
}
