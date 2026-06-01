#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub penalties: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: String,
    pub score: u32,
    pub penalties: u32,
}

pub fn rank_players(players: &[Player]) -> Vec<RankedPlayer> {
    let mut ordered = players.to_vec();
    ordered.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| a.penalties.cmp(&b.penalties))
    });

    let mut out = Vec::with_capacity(ordered.len());
    let mut last_score = None;
    let mut rank = 0;

    for (idx, p) in ordered.into_iter().enumerate() {
        if last_score != Some(p.score) {
            rank = idx + 1;
            last_score = Some(p.score);
        }
        out.push(RankedPlayer {
            rank,
            name: p.name,
            score: p.score,
            penalties: p.penalties,
        });
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: u32, penalties: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            penalties,
        }
    }

    #[test]
    fn ranks_by_score_then_penalty_then_name() {
        let ranked = rank_players(&[
            p("Zoe", 50, 3),
            p("Amy", 50, 1),
            p("Bob", 50, 1),
            p("Kai", 40, 0),
        ]);

        let summary: Vec<_> = ranked
            .into_iter()
            .map(|r| (r.rank, r.name, r.score, r.penalties))
            .collect();

        assert_eq!(
            summary,
            vec![
                (1, "Amy".to_string(), 50, 1),
                (1, "Bob".to_string(), 50, 1),
                (1, "Zoe".to_string(), 50, 3),
                (4, "Kai".to_string(), 40, 0),
            ]
        );
    }

    #[test]
    fn same_score_keeps_dense_rank_even_when_penalties_differ() {
        let ranked = rank_players(&[
            p("Nia", 70, 4),
            p("Ian", 70, 2),
            p("Omar", 65, 1),
            p("Pia", 65, 0),
        ]);

        let ranks: Vec<_> = ranked.into_iter().map(|r| (r.rank, r.name)).collect();
        assert_eq!(
            ranks,
            vec![
                (1, "Ian".to_string()),
                (1, "Nia".to_string()),
                (3, "Pia".to_string()),
                (3, "Omar".to_string()),
            ]
        );
    }
}
