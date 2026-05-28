#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub wins: u32,
    pub losses: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.wins
            .cmp(&a.wins)
            .then(a.losses.cmp(&b.losses))
            .then(b.name.cmp(&a.name))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(idx, p)| format!("{}. {} ({}-{})", idx + 1, p.name, p.wins, p.losses))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orders_by_wins_then_losses_then_name() {
        let players = vec![
            Player { name: "zoe", wins: 5, losses: 2 },
            Player { name: "amy", wins: 7, losses: 4 },
            Player { name: "bob", wins: 7, losses: 3 },
            Player { name: "cara", wins: 7, losses: 3 },
            Player { name: "dan", wins: 5, losses: 1 },
        ];

        let ranked = leaderboard(&players);
        assert_eq!(
            ranked,
            vec![
                "1. bob (7-3)",
                "2. cara (7-3)",
                "3. amy (7-4)",
                "4. dan (5-1)",
                "5. zoe (5-2)",
            ]
        );
    }

    #[test]
    fn alphabetical_tiebreak_is_ascending() {
        let players = vec![
            Player { name: "mia", wins: 2, losses: 2 },
            Player { name: "ava", wins: 2, losses: 2 },
            Player { name: "ivy", wins: 2, losses: 2 },
        ];

        let ranked = leaderboard(&players);
        assert_eq!(
            ranked,
            vec![
                "1. ava (2-2)",
                "2. ivy (2-2)",
                "3. mia (2-2)",
            ]
        );
    }
}
