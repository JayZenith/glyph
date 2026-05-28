#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn rank_players(players: &[Player]) -> Vec<RankedPlayer> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| b.wins.cmp(&a.wins))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(idx, p)| RankedPlayer {
            rank: idx + 1,
            name: p.name,
            score: p.score,
            wins: p.wins,
        })
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
        let ranked = rank_players(&[
            p("Zoe", 15, 3),
            p("Amy", 20, 1),
            p("Bob", 20, 4),
            p("Cal", 20, 4),
            p("Dan", 15, 5),
        ]);

        let names: Vec<_> = ranked.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["Bob", "Cal", "Amy", "Dan", "Zoe"]);
    }

    #[test]
    fn assigns_dense_ranks_for_exact_ties() {
        let ranked = rank_players(&[
            p("Amy", 20, 4),
            p("Bob", 20, 4),
            p("Cid", 18, 6),
            p("Dee", 18, 6),
            p("Eli", 10, 1),
        ]);

        let pairs: Vec<_> = ranked.into_iter().map(|r| (r.name, r.rank)).collect();
        assert_eq!(
            pairs,
            vec![
                ("Amy".to_string(), 1),
                ("Bob".to_string(), 1),
                ("Cid".to_string(), 2),
                ("Dee".to_string(), 2),
                ("Eli".to_string(), 3),
            ]
        );
    }
}
