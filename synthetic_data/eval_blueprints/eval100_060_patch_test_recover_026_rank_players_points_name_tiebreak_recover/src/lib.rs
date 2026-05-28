#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub points: u32,
    pub wins: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: String,
    pub points: u32,
    pub wins: u32,
}

pub fn rank_players(players: &[Player]) -> Vec<RankedPlayer> {
    let mut ordered = players.to_vec();
    ordered.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| a.name.cmp(&b.name))
    });

    ordered
        .into_iter()
        .enumerate()
        .map(|(idx, p)| RankedPlayer {
            rank: idx + 1,
            name: p.name,
            points: p.points,
            wins: p.wins,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, points: u32, wins: u32) -> Player {
        Player {
            name: name.to_string(),
            points,
            wins,
        }
    }

    #[test]
    fn sorts_by_points_then_wins_then_name() {
        let ranked = rank_players(&[
            p("Zoe", 12, 4),
            p("Ava", 12, 5),
            p("Mia", 9, 7),
            p("Bea", 12, 5),
        ]);

        let names: Vec<_> = ranked.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["Ava", "Bea", "Zoe", "Mia"]);
        let ranks: Vec<_> = ranked.iter().map(|r| r.rank).collect();
        assert_eq!(ranks, vec![1, 1, 3, 4]);
    }

    #[test]
    fn equal_points_and_wins_share_rank() {
        let ranked = rank_players(&[
            p("Noah", 8, 3),
            p("Liam", 8, 3),
            p("Emma", 7, 6),
        ]);

        assert_eq!(ranked[0].name, "Liam");
        assert_eq!(ranked[1].name, "Noah");
        assert_eq!(ranked[0].rank, 1);
        assert_eq!(ranked[1].rank, 1);
        assert_eq!(ranked[2].rank, 3);
    }
}
