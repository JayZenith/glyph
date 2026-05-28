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

pub fn leaderboard(players: &[Player]) -> Vec<RankedPlayer> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| b.wins.cmp(&a.wins))
    });

    let mut out = Vec::with_capacity(items.len());
    let mut last_score_wins = None;
    let mut last_rank = 0usize;

    for (idx, p) in items.into_iter().enumerate() {
        let key = (p.score, p.wins);
        let rank = if Some(key) == last_score_wins {
            last_rank
        } else {
            idx + 1
        };
        last_score_wins = Some(key);
        last_rank = rank;
        out.push(RankedPlayer {
            rank,
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
    fn orders_by_score_then_wins_then_name() {
        let ranked = leaderboard(&[
            p("Zoe", 20, 3),
            p("Amy", 20, 5),
            p("Bob", 20, 5),
            p("Eli", 18, 9),
        ]);

        let names: Vec<_> = ranked.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["Amy", "Bob", "Zoe", "Eli"]);
    }

    #[test]
    fn ties_share_rank_and_next_rank_skips() {
        let ranked = leaderboard(&[
            p("Bob", 30, 4),
            p("Amy", 30, 4),
            p("Cid", 25, 7),
            p("Dee", 20, 1),
        ]);

        let pairs: Vec<_> = ranked.iter().map(|r| (r.name.as_str(), r.rank)).collect();
        assert_eq!(pairs, vec![("Amy", 1), ("Bob", 1), ("Cid", 3), ("Dee", 4)]);
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let ranked = leaderboard(&[]);
        assert!(ranked.is_empty());
    }
}
