use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
}

impl Player {
    fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }

    fn played(&self) -> u32 {
        self.wins + self.draws + self.losses
    }
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by_key(|p| (Reverse(p.points()), p.name));

    rows.into_iter()
        .enumerate()
        .map(|(idx, p)| {
            format!(
                "{}. {} [{} pts, {} gp]",
                idx + 1,
                p.name,
                p.points(),
                p.played()
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_points_then_fewer_games_then_name_and_uses_dense_ranks() {
        let players = vec![
            Player { name: "Bears", wins: 3, draws: 0, losses: 0 },
            Player { name: "Arrows", wins: 2, draws: 3, losses: 0 },
            Player { name: "Cobras", wins: 2, draws: 0, losses: 4 },
            Player { name: "Blaze", wins: 2, draws: 0, losses: 4 },
            Player { name: "Dragons", wins: 1, draws: 0, losses: 0 },
            Player { name: "Eagles", wins: 1, draws: 0, losses: 0 },
        ];

        assert_eq!(
            leaderboard(&players),
            vec![
                "1. Bears [9 pts, 3 gp]",
                "2. Arrows [9 pts, 5 gp]",
                "3. Blaze [6 pts, 6 gp]",
                "3. Cobras [6 pts, 6 gp]",
                "4. Dragons [3 pts, 1 gp]",
                "4. Eagles [3 pts, 1 gp]",
            ]
        );
    }

    #[test]
    fn exact_ties_share_rank_after_non_tied_rows() {
        let players = vec![
            Player { name: "Lions", wins: 4, draws: 0, losses: 0 },
            Player { name: "Meteors", wins: 3, draws: 0, losses: 1 },
            Player { name: "Nova", wins: 3, draws: 0, losses: 1 },
            Player { name: "Orcas", wins: 2, draws: 2, losses: 0 },
        ];

        assert_eq!(
            leaderboard(&players),
            vec![
                "1. Lions [12 pts, 4 gp]",
                "2. Meteors [9 pts, 4 gp]",
                "2. Nova [9 pts, 4 gp]",
                "3. Orcas [8 pts, 4 gp]",
            ]
        );
    }
}
