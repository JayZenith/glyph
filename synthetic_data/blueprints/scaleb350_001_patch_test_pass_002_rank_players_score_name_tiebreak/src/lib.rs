#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: String,
    pub score: u32,
}

pub fn rank_players(players: &[Player]) -> Vec<RankedPlayer> {
    let mut entries = players.to_vec();
    entries.sort_by(|a, b| a.score.cmp(&b.score).then_with(|| a.name.cmp(&b.name)));

    let mut out = Vec::with_capacity(entries.len());
    for (idx, p) in entries.into_iter().enumerate() {
        out.push(RankedPlayer {
            rank: idx + 1,
            name: p.name,
            score: p.score,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
        }
    }

    #[test]
    fn sorts_by_score_desc_then_name_asc() {
        let ranked = rank_players(&[
            p("zoe", 12),
            p("amy", 20),
            p("mike", 12),
            p("bob", 20),
        ]);

        let pairs: Vec<(usize, String, u32)> = ranked
            .into_iter()
            .map(|r| (r.rank, r.name, r.score))
            .collect();

        assert_eq!(
            pairs,
            vec![
                (1, "amy".to_string(), 20),
                (1, "bob".to_string(), 20),
                (3, "mike".to_string(), 12),
                (3, "zoe".to_string(), 12),
            ]
        );
    }

    #[test]
    fn leaves_gaps_after_ties() {
        let ranked = rank_players(&[
            p("kai", 30),
            p("ivy", 30),
            p("uma", 25),
            p("abe", 25),
            p("neo", 10),
        ]);

        let ranks: Vec<usize> = ranked.into_iter().map(|r| r.rank).collect();
        assert_eq!(ranks, vec![1, 1, 3, 3, 5]);
    }
}
