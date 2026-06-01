use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub score: u32,
    pub penalties: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<(usize, &'static str, u32, u32)> {
    let mut rows = players.to_vec();
    rows.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| b.penalties.cmp(&a.penalties))
            .then_with(|| b.name.cmp(a.name))
    });

    let mut out = Vec::new();
    let mut rank = 1usize;

    for (i, p) in rows.iter().enumerate() {
        if i > 0 {
            let prev = &rows[i - 1];
            if prev.score != p.score || prev.penalties != p.penalties {
                rank += 1;
            }
        }
        out.push((rank, p.name, p.score, p.penalties));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::{leaderboard, Player};

    #[test]
    fn sorts_by_score_then_penalties_then_name() {
        let players = vec![
            Player { name: "zoe", score: 20, penalties: 1 },
            Player { name: "amy", score: 20, penalties: 1 },
            Player { name: "bob", score: 20, penalties: 0 },
            Player { name: "ian", score: 18, penalties: 0 },
        ];

        let board = leaderboard(&players);
        let names: Vec<_> = board.into_iter().map(|row| row.1).collect();
        assert_eq!(names, vec!["bob", "amy", "zoe", "ian"]);
    }

    #[test]
    fn equal_score_and_penalties_share_rank_with_gaps() {
        let players = vec![
            Player { name: "mia", score: 30, penalties: 1 },
            Player { name: "ava", score: 30, penalties: 1 },
            Player { name: "eli", score: 28, penalties: 0 },
            Player { name: "noa", score: 28, penalties: 0 },
            Player { name: "rex", score: 25, penalties: 3 },
        ];

        let board = leaderboard(&players);
        let ranks: Vec<_> = board.into_iter().map(|row| (row.0, row.1)).collect();
        assert_eq!(ranks, vec![(1, "ava"), (1, "mia"), (3, "eli"), (3, "noa"), (5, "rex")]);
    }
}
