#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub wins: u32,
    pub losses: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: String,
    pub wins: u32,
    pub losses: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<RankedPlayer> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.wins
            .cmp(&a.wins)
            .then(a.losses.cmp(&b.losses))
            .then(a.name.cmp(&b.name))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(i, p)| RankedPlayer {
            rank: i + 1,
            name: p.name,
            wins: p.wins,
            losses: p.losses,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, wins: u32, losses: u32) -> Player {
        Player {
            name: name.to_string(),
            wins,
            losses,
        }
    }

    #[test]
    fn orders_by_wins_then_fewer_losses_then_name_desc() {
        let ranked = leaderboard(&[
            p("Ava", 7, 3),
            p("Zoe", 7, 3),
            p("Mia", 7, 1),
            p("Bea", 6, 0),
        ]);

        let names: Vec<_> = ranked.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["Mia", "Zoe", "Ava", "Bea"]);
    }

    #[test]
    fn tied_records_share_rank_and_next_rank_skips() {
        let ranked = leaderboard(&[
            p("Zoe", 8, 2),
            p("Ava", 8, 2),
            p("Mia", 7, 0),
            p("Nia", 7, 0),
            p("Bea", 5, 5),
        ]);

        let pairs: Vec<_> = ranked.iter().map(|r| (r.rank, r.name.as_str())).collect();
        assert_eq!(pairs, vec![(1, "Zoe"), (1, "Ava"), (3, "Nia"), (3, "Mia"), (5, "Bea")]);
    }
}
