use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by_key(|p| (Reverse(p.score), p.wins, p.name.clone()));
    items.into_iter().map(|p| p.name).collect()
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
        let players = vec![
            p("zoe", 20, 3),
            p("amy", 25, 1),
            p("bob", 25, 4),
            p("cara", 25, 4),
            p("dan", 20, 5),
        ];

        assert_eq!(
            leaderboard(&players),
            vec!["bob", "cara", "amy", "dan", "zoe"]
        );
    }

    #[test]
    fn name_breaks_full_ties_alphabetically() {
        let players = vec![
            p("mia", 10, 2),
            p("abe", 10, 2),
            p("ivy", 10, 2),
        ];

        assert_eq!(leaderboard(&players), vec!["abe", "ivy", "mia"]);
    }
}
