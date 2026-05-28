#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub wins: u32,
    pub losses: u32,
    pub points: i32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by(|a, b| {
        b.wins
            .cmp(&a.wins)
            .then_with(|| a.losses.cmp(&b.losses))
            .then_with(|| a.name.cmp(&b.name))
    });

    rows.into_iter()
        .enumerate()
        .map(|(idx, p)| format!("{}. {} ({}-{}, {} pts)", idx + 1, p.name, p.wins, p.losses, p.points))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_by_wins_then_points_then_fewer_losses_then_name() {
        let players = vec![
            Player { name: "Zoe", wins: 8, losses: 3, points: 15 },
            Player { name: "Amy", wins: 8, losses: 2, points: 15 },
            Player { name: "Bob", wins: 8, losses: 1, points: 18 },
            Player { name: "Cara", wins: 9, losses: 5, points: 10 },
        ];

        let board = leaderboard(&players);
        assert_eq!(board, vec![
            "1. Cara (9-5, 10 pts)",
            "2. Bob (8-1, 18 pts)",
            "3. Amy (8-2, 15 pts)",
            "4. Zoe (8-3, 15 pts)",
        ]);
    }

    #[test]
    fn preserves_shared_rank_numbers_for_tied_records() {
        let players = vec![
            Player { name: "Ivy", wins: 7, losses: 2, points: 12 },
            Player { name: "Eli", wins: 7, losses: 2, points: 12 },
            Player { name: "Moe", wins: 6, losses: 1, points: 30 },
            Player { name: "Nia", wins: 6, losses: 1, points: 25 },
            Player { name: "Uma", wins: 5, losses: 0, points: 40 },
        ];

        let board = leaderboard(&players);
        assert_eq!(board, vec![
            "1. Eli (7-2, 12 pts)",
            "1. Ivy (7-2, 12 pts)",
            "3. Moe (6-1, 30 pts)",
            "4. Nia (6-1, 25 pts)",
            "5. Uma (5-0, 40 pts)",
        ]);
    }
}
