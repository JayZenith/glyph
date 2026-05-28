#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .reverse()
            .then(a.wins.cmp(&b.wins).reverse())
            .then(a.name.cmp(&b.name).reverse())
    });

    rows.into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} ({} pts, {} wins)", i + 1, p.name, p.score, p.wins))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: u32, wins: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            wins,
        }
    }

    #[test]
    fn sorts_by_score_then_wins_then_name() {
        let players = vec![
            p("Zoe", 30, 4),
            p("Amy", 50, 1),
            p("Cara", 50, 3),
            p("Bea", 50, 3),
            p("Dan", 30, 9),
        ];

        let lines = leaderboard(&players);
        assert_eq!(
            lines,
            vec![
                "1. Bea (50 pts, 3 wins)",
                "2. Cara (50 pts, 3 wins)",
                "3. Amy (50 pts, 1 wins)",
                "4. Dan (30 pts, 9 wins)",
                "5. Zoe (30 pts, 4 wins)",
            ]
        );
    }

    #[test]
    fn exact_ties_keep_all_rows_with_alphabetical_names() {
        let players = vec![
            p("Lia", 10, 2),
            p("Ava", 10, 2),
            p("Mia", 10, 2),
        ];

        let lines = leaderboard(&players);
        assert_eq!(
            lines,
            vec![
                "1. Ava (10 pts, 2 wins)",
                "2. Lia (10 pts, 2 wins)",
                "3. Mia (10 pts, 2 wins)",
            ]
        );
    }
}
