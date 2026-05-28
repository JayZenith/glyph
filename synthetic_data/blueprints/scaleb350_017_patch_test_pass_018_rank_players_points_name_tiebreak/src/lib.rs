#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub points: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.wins.cmp(&b.wins))
            .then(a.name.cmp(b.name))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(idx, p)| format!("{}. {} ({} pts, {} wins)", idx + 1, p.name, p.points, p.wins))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_points_then_wins_then_name() {
        let players = vec![
            Player { name: "Zoe", points: 12, wins: 4 },
            Player { name: "Amy", points: 12, wins: 5 },
            Player { name: "Bob", points: 15, wins: 2 },
            Player { name: "Cara", points: 12, wins: 5 },
        ];

        let lines = leaderboard(&players);
        let expected = vec![
            "1. Bob (15 pts, 2 wins)",
            "2. Amy (12 pts, 5 wins)",
            "3. Cara (12 pts, 5 wins)",
            "4. Zoe (12 pts, 4 wins)",
        ];

        assert_eq!(lines, expected);
    }

    #[test]
    fn name_is_ascending_final_tiebreak() {
        let players = vec![
            Player { name: "Nina", points: 8, wins: 3 },
            Player { name: "Adam", points: 8, wins: 3 },
            Player { name: "Mia", points: 8, wins: 3 },
        ];

        let lines = leaderboard(&players);
        let expected = vec![
            "1. Adam (8 pts, 3 wins)",
            "2. Mia (8 pts, 3 wins)",
            "3. Nina (8 pts, 3 wins)",
        ];

        assert_eq!(lines, expected);
    }

    #[test]
    fn empty_input_returns_empty_board() {
        let lines = leaderboard(&[]);
        assert!(lines.is_empty());
    }
}
