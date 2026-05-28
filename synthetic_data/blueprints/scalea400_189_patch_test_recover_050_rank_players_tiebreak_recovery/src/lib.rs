#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub points: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.wins.cmp(&b.wins))
            .then(a.name.cmp(&b.name))
    });

    rows.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::{leaderboard, Player};

    fn p(name: &str, points: u32, wins: u32) -> Player {
        Player {
            name: name.to_string(),
            points,
            wins,
        }
    }

    #[test]
    fn sorts_by_points_then_wins_then_name() {
        let players = vec![
            p("Mira", 12, 5),
            p("Ava", 20, 7),
            p("Zoe", 20, 7),
            p("Liam", 20, 9),
            p("Noah", 12, 8),
        ];

        assert_eq!(
            leaderboard(&players),
            vec!["Liam", "Ava", "Zoe", "Noah", "Mira"]
        );
    }

    #[test]
    fn keeps_only_best_entry_per_name_before_sorting() {
        let players = vec![
            p("Kai", 10, 3),
            p("Ivy", 15, 4),
            p("Kai", 14, 2),
            p("Ivy", 15, 6),
            p("Bea", 15, 6),
        ];

        assert_eq!(leaderboard(&players), vec!["Bea", "Ivy", "Kai"]);
    }
}
