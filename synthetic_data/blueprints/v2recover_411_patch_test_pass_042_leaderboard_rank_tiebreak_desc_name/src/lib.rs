use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub wins: u32,
    pub points: i32,
}

pub fn leaderboard(players: &[Player]) -> Vec<&'static str> {
    let mut rows: Vec<&Player> = players.iter().collect();
    rows.sort_by_key(|p| (Reverse(p.wins), p.points, p.name));
    rows.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn higher_wins_rank_first() {
        let players = vec![
            Player { name: "Zoe", wins: 2, points: 50 },
            Player { name: "Amy", wins: 4, points: 10 },
            Player { name: "Bob", wins: 3, points: 99 },
        ];
        assert_eq!(leaderboard(&players), vec!["Amy", "Bob", "Zoe"]);
    }

    #[test]
    fn ties_on_wins_use_points_descending() {
        let players = vec![
            Player { name: "Amy", wins: 3, points: 12 },
            Player { name: "Bob", wins: 3, points: 20 },
            Player { name: "Cara", wins: 3, points: 15 },
        ];
        assert_eq!(leaderboard(&players), vec!["Bob", "Cara", "Amy"]);
    }

    #[test]
    fn ties_on_wins_and_points_use_name_ascending() {
        let players = vec![
            Player { name: "Mia", wins: 5, points: 7 },
            Player { name: "Ava", wins: 5, points: 7 },
            Player { name: "Lia", wins: 5, points: 7 },
        ];
        assert_eq!(leaderboard(&players), vec!["Ava", "Lia", "Mia"]);
    }

    #[test]
    fn mixed_tiebreaks_work_together() {
        let players = vec![
            Player { name: "Nia", wins: 4, points: 8 },
            Player { name: "Eli", wins: 4, points: 11 },
            Player { name: "Ari", wins: 4, points: 11 },
            Player { name: "Bea", wins: 2, points: 99 },
        ];
        assert_eq!(leaderboard(&players), vec!["Ari", "Eli", "Nia", "Bea"]);
    }
}
