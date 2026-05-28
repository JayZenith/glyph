use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by_key(|p| (Reverse(p.score), p.name.clone(), Reverse(p.wins)));
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
    fn sorts_by_score_descending() {
        let players = vec![p("Ivy", 30, 1), p("Bea", 40, 0), p("Kai", 35, 5)];
        assert_eq!(leaderboard(&players), vec!["Bea", "Kai", "Ivy"]);
    }

    #[test]
    fn breaks_score_ties_by_wins_descending() {
        let players = vec![p("Lia", 20, 1), p("Moe", 20, 4), p("Nia", 20, 3)];
        assert_eq!(leaderboard(&players), vec!["Moe", "Nia", "Lia"]);
    }

    #[test]
    fn breaks_full_ties_by_name_ascending() {
        let players = vec![p("Zed", 50, 2), p("Ada", 50, 2), p("Bo", 50, 2)];
        assert_eq!(leaderboard(&players), vec!["Ada", "Bo", "Zed"]);
    }

    #[test]
    fn applies_all_tiebreakers_together() {
        let players = vec![
            p("Uma", 42, 3),
            p("Ava", 42, 5),
            p("Eli", 50, 0),
            p("Noa", 42, 5),
            p("Ian", 42, 4),
        ];
        assert_eq!(leaderboard(&players), vec!["Eli", "Ava", "Noa", "Ian", "Uma"]);
    }
}
