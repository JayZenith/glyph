#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub wins: u32,
    pub goal_diff: i32,
    pub scored: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<&'static str> {
    let mut rows: Vec<&Player> = players.iter().collect();
    rows.sort_by(|a, b| {
        a.wins
            .cmp(&b.wins)
            .then(a.goal_diff.cmp(&b.goal_diff))
            .then(a.scored.cmp(&b.scored))
            .then(a.name.cmp(&b.name))
    });
    rows.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &'static str, wins: u32, goal_diff: i32, scored: u32) -> Player {
        Player {
            name,
            wins,
            goal_diff,
            scored,
        }
    }

    #[test]
    fn sorts_by_wins_then_goal_diff_then_scored_desc() {
        let players = vec![
            p("Bears", 3, 2, 9),
            p("Arrows", 5, 1, 8),
            p("Comets", 5, 4, 6),
            p("Dragons", 5, 4, 10),
        ];

        assert_eq!(
            leaderboard(&players),
            vec!["Dragons", "Comets", "Arrows", "Bears"]
        );
    }

    #[test]
    fn final_tiebreak_is_name_ascending() {
        let players = vec![
            p("Zephyrs", 4, 3, 7),
            p("Astros", 4, 3, 7),
            p("Bolts", 4, 3, 7),
        ];

        assert_eq!(leaderboard(&players), vec!["Astros", "Bolts", "Zephyrs"]);
    }

    #[test]
    fn handles_negative_goal_diff_without_flipping_name_order() {
        let players = vec![
            p("Knights", 2, -1, 5),
            p("Lynx", 2, -3, 9),
            p("Meteors", 2, -1, 4),
        ];

        assert_eq!(leaderboard(&players), vec!["Knights", "Meteors", "Lynx"]);
    }
}
