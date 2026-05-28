use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub penalty: u32,
}

pub fn leaderboard(mut players: Vec<Player>) -> Vec<Player> {
    players.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .reverse()
            .then(a.penalty.cmp(&b.penalty))
            .then(b.name.cmp(&a.name))
    });
    players
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: u32, penalty: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            penalty,
        }
    }

    #[test]
    fn sorts_by_score_then_penalty_then_name() {
        let ranked = leaderboard(vec![
            p("zoe", 15, 3),
            p("amy", 20, 5),
            p("bob", 20, 5),
            p("dax", 20, 2),
            p("eve", 15, 1),
        ]);

        let names: Vec<_> = ranked.into_iter().map(|p| p.name).collect();
        assert_eq!(names, vec!["dax", "amy", "bob", "eve", "zoe"]);
    }

    #[test]
    fn deterministic_for_full_ties_except_name() {
        let ranked = leaderboard(vec![
            p("mila", 8, 4),
            p("anna", 8, 4),
            p("olaf", 8, 4),
        ]);

        let names: Vec<_> = ranked.into_iter().map(|p| p.name).collect();
        assert_eq!(names, vec!["anna", "mila", "olaf"]);
    }
}
