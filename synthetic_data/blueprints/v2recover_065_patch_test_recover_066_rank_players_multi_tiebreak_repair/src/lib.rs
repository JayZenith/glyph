use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub points: u32,
    pub wins: u32,
    pub penalties: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.wins.cmp(&b.wins))
            .then(a.penalties.cmp(&b.penalties))
            .then(a.name.cmp(b.name))
    });

    rows.into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} [{}-{}-{}]", i + 1, p.name, p.points, p.wins, p.penalties))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{leaderboard, Player};

    #[test]
    fn sorts_by_points_then_wins_then_lower_penalties_then_name() {
        let players = vec![
            Player { name: "Zoey", points: 10, wins: 4, penalties: 2 },
            Player { name: "Amy", points: 12, wins: 3, penalties: 5 },
            Player { name: "Cara", points: 12, wins: 5, penalties: 7 },
            Player { name: "Bea", points: 12, wins: 5, penalties: 1 },
            Player { name: "Dan", points: 12, wins: 5, penalties: 1 },
        ];

        let board = leaderboard(&players);
        let names: Vec<&str> = board
            .iter()
            .map(|line| line.split_once(". ").unwrap().1.split_once(" [").unwrap().0)
            .collect();

        assert_eq!(names, vec!["Bea", "Dan", "Cara", "Amy", "Zoey"]);
    }

    #[test]
    fn ranking_numbers_share_places_for_identical_scores() {
        let players = vec![
            Player { name: "Mia", points: 15, wins: 6, penalties: 2 },
            Player { name: "Ava", points: 15, wins: 6, penalties: 2 },
            Player { name: "Nia", points: 14, wins: 7, penalties: 0 },
            Player { name: "Eli", points: 10, wins: 3, penalties: 9 },
        ];

        let board = leaderboard(&players);
        assert_eq!(
            board,
            vec![
                "1. Ava [15-6-2]".to_string(),
                "1. Mia [15-6-2]".to_string(),
                "3. Nia [14-7-0]".to_string(),
                "4. Eli [10-3-9]".to_string(),
            ]
        );
    }

    #[test]
    fn empty_input_returns_empty_board() {
        let board = leaderboard(&[]);
        assert!(board.is_empty());
    }
}
