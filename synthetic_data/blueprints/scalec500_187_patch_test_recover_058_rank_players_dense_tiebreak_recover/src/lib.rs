#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: i32,
    pub wins: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedPlayer {
    pub rank: usize,
    pub name: String,
    pub score: i32,
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
    let mut last_score = None;
    let mut rank = 0usize;

    for (i, p) in items.into_iter().enumerate() {
        if last_score != Some(p.score) {
            rank = i + 1;
            last_score = Some(p.score);
        }
        out.push(RankedPlayer {
            rank,
            name: p.name,
            score: p.score,
            wins: p.wins,
        });
    }

    out.dedup_by(|a, b| a.name == b.name);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: i32, wins: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            wins,
        }
    }

    #[test]
    fn sorts_by_score_then_wins_then_name() {
        let got = leaderboard(&[
            p("Zed", 30, 1),
            p("Amy", 40, 2),
            p("Bob", 40, 5),
            p("Cara", 40, 5),
            p("Dan", 30, 9),
        ]);

        let names: Vec<_> = got.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["Bob", "Cara", "Amy", "Dan", "Zed"]);
    }

    #[test]
    fn uses_dense_ranks_for_ties_on_score_and_wins() {
        let got = leaderboard(&[
            p("Amy", 50, 3),
            p("Bea", 50, 3),
            p("Cal", 40, 9),
            p("Dee", 40, 1),
            p("Eli", 10, 0),
        ]);

        let pairs: Vec<_> = got.iter().map(|r| (r.name.as_str(), r.rank)).collect();
        assert_eq!(pairs, vec![("Amy", 1), ("Bea", 1), ("Cal", 2), ("Dee", 3), ("Eli", 4)]);
    }

    #[test]
    fn keeps_duplicate_names_as_separate_rows() {
        let got = leaderboard(&[
            p("Alex", 20, 2),
            p("Alex", 10, 9),
            p("Blair", 15, 4),
        ]);

        let rows: Vec<_> = got.iter().map(|r| (r.name.as_str(), r.score, r.rank)).collect();
        assert_eq!(rows, vec![("Alex", 20, 1), ("Blair", 15, 2), ("Alex", 10, 3)]);
    }
}
