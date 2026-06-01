#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<RankedPlayer> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| b.wins.cmp(&a.wins))
    });

    let mut out = Vec::new();
    for (i, p) in items.into_iter().enumerate() {
        out.push(RankedPlayer {
            rank: i + 1,
            name: p.name,
            score: p.score,
            wins: p.wins,
        });
    }
    out
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
    fn sorts_by_score_then_wins_then_name_case_insensitive() {
        let ranked = leaderboard(&[
            p("zoe", 20, 4),
            p("Amy", 20, 5),
            p("bob", 20, 5),
            p("alex", 18, 9),
        ]);
        let names: Vec<_> = ranked.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["Amy", "bob", "zoe", "alex"]);
        let ranks: Vec<_> = ranked.iter().map(|r| r.rank).collect();
        assert_eq!(ranks, vec![1, 1, 3, 4]);
    }

    #[test]
    fn duplicate_records_are_removed_but_distinct_names_remain() {
        let ranked = leaderboard(&[
            p("Kai", 15, 2),
            p("Kai", 15, 2),
            p("kai", 15, 2),
            p("Mia", 15, 1),
        ]);
        let rows: Vec<_> = ranked
            .iter()
            .map(|r| (r.rank, r.name.as_str(), r.score, r.wins))
            .collect();
        assert_eq!(
            rows,
            vec![(1, "Kai", 15, 2), (1, "kai", 15, 2), (3, "Mia", 15, 1)]
        );
    }

    #[test]
    fn later_positions_skip_after_ties() {
        let ranked = leaderboard(&[
            p("Nia", 30, 1),
            p("Omar", 30, 1),
            p("Pia", 29, 9),
            p("Qin", 28, 9),
        ]);
        let ranks: Vec<_> = ranked.iter().map(|r| (r.name.as_str(), r.rank)).collect();
        assert_eq!(ranks, vec![("Nia", 1), ("Omar", 1), ("Pia", 3), ("Qin", 4)]);
    }
}
