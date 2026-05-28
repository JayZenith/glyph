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
    let mut rows = players.to_vec();
    rows.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(a.name.cmp(&b.name))
            .then(b.wins.cmp(&a.wins))
    });

    let mut out = Vec::with_capacity(rows.len());
    for (i, p) in rows.into_iter().enumerate() {
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
    fn sorts_by_score_then_wins_then_name() {
        let ranked = rank_players(&[
            p("zoe", 30, 2),
            p("amy", 30, 5),
            p("bob", 40, 1),
            p("cara", 30, 5),
        ]);

        let names: Vec<_> = ranked.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["bob", "amy", "cara", "zoe"]);
        let ranks: Vec<_> = ranked.iter().map(|r| r.rank).collect();
        assert_eq!(ranks, vec![1, 2, 2, 4]);
    }

    #[test]
    fn ties_share_rank_but_next_rank_skips_ahead() {
        let ranked = rank_players(&[
            p("ivy", 50, 3),
            p("abe", 50, 3),
            p("mia", 45, 8),
            p("neo", 45, 1),
            p("kai", 45, 1),
        ]);

        let names: Vec<_> = ranked.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["abe", "ivy", "mia", "kai", "neo"]);
        let ranks: Vec<_> = ranked.iter().map(|r| r.rank).collect();
        assert_eq!(ranks, vec![1, 1, 3, 4, 4]);
    }
}
