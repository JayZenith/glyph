#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub points: u32,
    pub wins: u32,
    pub last_win_day: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut ranked = players.to_vec();
    ranked.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| a.wins.cmp(&b.wins))
            .then_with(|| a.last_win_day.cmp(&b.last_win_day))
            .then_with(|| a.name.cmp(&b.name))
    });
    ranked.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, points: u32, wins: u32, last_win_day: u32) -> Player {
        Player {
            name: name.to_string(),
            points,
            wins,
            last_win_day,
        }
    }

    #[test]
    fn ranks_by_points_then_wins_then_recent_win_then_name() {
        let players = vec![
            p("Drew", 12, 5, 8),
            p("Bea", 12, 7, 3),
            p("Ava", 12, 7, 9),
            p("Cara", 15, 2, 1),
        ];

        assert_eq!(leaderboard(&players), vec!["Cara", "Ava", "Bea", "Drew"]);
    }

    #[test]
    fn name_is_final_ascending_tiebreak() {
        let players = vec![
            p("Zoe", 9, 4, 6),
            p("Amy", 9, 4, 6),
            p("Ian", 9, 4, 6),
        ];

        assert_eq!(leaderboard(&players), vec!["Amy", "Ian", "Zoe"]);
    }
}
