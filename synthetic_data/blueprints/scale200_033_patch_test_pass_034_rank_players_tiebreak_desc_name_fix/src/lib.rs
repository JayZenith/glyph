use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by_key(|p| (Reverse(p.score), p.wins, p.name.clone()));
    rows.into_iter().map(|p| p.name).collect()
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
            p("zoe", 12, 4),
            p("amy", 12, 6),
            p("bob", 9, 9),
            p("cara", 12, 6),
            p("drew", 12, 4),
        ];

        assert_eq!(
            leaderboard(&players),
            vec!["amy", "cara", "drew", "zoe", "bob"]
        );
    }

    #[test]
    fn name_breaks_full_ties_alphabetically() {
        let players = vec![
            p("mia", 7, 2),
            p("ava", 7, 2),
            p("noah", 7, 2),
        ];

        assert_eq!(leaderboard(&players), vec!["ava", "mia", "noah"]);
    }

    #[test]
    fn higher_wins_beat_lower_wins_when_scores_match() {
        let players = vec![p("ivy", 15, 1), p("leo", 15, 3), p("max", 14, 9)];

        assert_eq!(leaderboard(&players), vec!["leo", "ivy", "max"]);
    }
}
