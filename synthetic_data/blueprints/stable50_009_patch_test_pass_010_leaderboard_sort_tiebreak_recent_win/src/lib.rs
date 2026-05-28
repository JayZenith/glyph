use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub wins: u32,
    pub losses: u32,
    pub last_win_day: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by_key(|p| (Reverse(p.wins), p.losses, p.name.clone()));
    rows.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, wins: u32, losses: u32, last_win_day: u32) -> Player {
        Player {
            name: name.to_string(),
            wins,
            losses,
            last_win_day,
        }
    }

    #[test]
    fn sorts_by_wins_then_losses_then_recent_win_then_name() {
        let players = vec![
            p("Mia", 7, 2, 15),
            p("Ava", 7, 2, 20),
            p("Zoe", 7, 2, 20),
            p("Eli", 8, 5, 3),
            p("Noah", 7, 1, 9),
        ];

        let names = leaderboard(&players);
        assert_eq!(names, vec!["Eli", "Noah", "Ava", "Zoe", "Mia"]);
    }

    #[test]
    fn name_only_breaks_full_ties() {
        let players = vec![
            p("Cara", 4, 4, 11),
            p("Ben", 4, 4, 11),
            p("Dax", 4, 4, 9),
        ];

        let names = leaderboard(&players);
        assert_eq!(names, vec!["Ben", "Cara", "Dax"]);
    }
}
