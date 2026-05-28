#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub wins: u32,
    pub losses: u32,
    pub points_for: i32,
    pub points_against: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: &'static str,
    pub wins: u32,
    pub losses: u32,
    pub diff: i32,
}

pub fn rank_players(players: &[Player]) -> Vec<RankedPlayer> {
    let mut rows: Vec<_> = players
        .iter()
        .map(|p| RankedPlayer {
            rank: 0,
            name: p.name,
            wins: p.wins,
            losses: p.losses,
            diff: p.points_for - p.points_against,
        })
        .collect();

    rows.sort_by(|a, b| {
        b.wins
            .cmp(&a.wins)
            .then_with(|| b.diff.cmp(&a.diff))
            .then_with(|| b.losses.cmp(&a.losses))
            .then_with(|| b.name.cmp(&a.name))
    });

    for (i, row) in rows.iter_mut().enumerate() {
        row.rank = i + 1;
    }

    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_wins_then_fewer_losses_then_diff_then_name() {
        let players = vec![
            Player { name: "Jets", wins: 8, losses: 4, points_for: 280, points_against: 250 },
            Player { name: "Aces", wins: 8, losses: 3, points_for: 260, points_against: 255 },
            Player { name: "Bears", wins: 8, losses: 3, points_for: 300, points_against: 280 },
            Player { name: "Comets", wins: 8, losses: 3, points_for: 220, points_against: 200 },
            Player { name: "Dragons", wins: 7, losses: 2, points_for: 250, points_against: 200 },
        ];

        let ranked = rank_players(&players);
        let names: Vec<_> = ranked.iter().map(|r| r.name).collect();
        assert_eq!(names, vec!["Bears", "Comets", "Aces", "Jets", "Dragons"]);
    }

    #[test]
    fn ties_share_rank_and_next_rank_skips_ahead() {
        let players = vec![
            Player { name: "Aces", wins: 9, losses: 2, points_for: 310, points_against: 280 },
            Player { name: "Blaze", wins: 9, losses: 2, points_for: 300, points_against: 270 },
            Player { name: "Cougars", wins: 7, losses: 5, points_for: 290, points_against: 260 },
            Player { name: "Dynamo", wins: 7, losses: 5, points_for: 280, points_against: 250 },
            Player { name: "Express", wins: 6, losses: 6, points_for: 240, points_against: 245 },
        ];

        let ranked = rank_players(&players);
        let pairs: Vec<_> = ranked.iter().map(|r| (r.name, r.rank)).collect();
        assert_eq!(pairs, vec![
            ("Aces", 1),
            ("Blaze", 1),
            ("Cougars", 3),
            ("Dynamo", 3),
            ("Express", 5),
        ]);
    }

    #[test]
    fn exact_ties_are_alphabetical_only() {
        let players = vec![
            Player { name: "Zephyrs", wins: 5, losses: 5, points_for: 210, points_against: 200 },
            Player { name: "Aurora", wins: 5, losses: 5, points_for: 205, points_against: 195 },
            Player { name: "Bolts", wins: 5, losses: 5, points_for: 190, points_against: 180 },
        ];

        let ranked = rank_players(&players);
        let names: Vec<_> = ranked.iter().map(|r| r.name).collect();
        let ranks: Vec<_> = ranked.iter().map(|r| r.rank).collect();
        assert_eq!(names, vec!["Aurora", "Bolts", "Zephyrs"]);
        assert_eq!(ranks, vec![1, 1, 1]);
    }
}
