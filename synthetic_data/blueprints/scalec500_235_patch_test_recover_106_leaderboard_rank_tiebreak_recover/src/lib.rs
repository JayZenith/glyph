#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| a.score.cmp(&b.score));

    items
        .into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} [{}|{}]", i + 1, p.name, p.score, p.wins))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_score_desc_then_wins_desc_then_name_asc() {
        let players = vec![
            Player { name: "zoe", score: 12, wins: 1 },
            Player { name: "amy", score: 20, wins: 2 },
            Player { name: "bob", score: 20, wins: 5 },
            Player { name: "cara", score: 20, wins: 5 },
        ];

        let got = leaderboard(&players);
        let want = vec![
            "1. bob [20|5]",
            "2. cara [20|5]",
            "3. amy [20|2]",
            "4. zoe [12|1]",
        ];

        assert_eq!(got, want);
    }

    #[test]
    fn stable_numbering_after_tiebreaks() {
        let players = vec![
            Player { name: "ivy", score: 8, wins: 4 },
            Player { name: "ada", score: 8, wins: 4 },
            Player { name: "max", score: 10, wins: 1 },
        ];

        let got = leaderboard(&players);
        let want = vec![
            "1. max [10|1]",
            "2. ada [8|4]",
            "3. ivy [8|4]",
        ];

        assert_eq!(got, want);
    }
}
