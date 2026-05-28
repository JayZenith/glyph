use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub points: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| match b.points.cmp(&a.points) {
        Ordering::Equal => match b.wins.cmp(&a.wins) {
            Ordering::Equal => a.name.cmp(b.name),
            other => other,
        },
        other => other,
    });

    items
        .into_iter()
        .enumerate()
        .map(|(idx, p)| format!("{}. {} ({}, {})", idx + 1, p.name, p.points, p.wins))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{leaderboard, Player};

    #[test]
    fn sorts_by_points_then_wins_then_name() {
        let players = vec![
            Player { name: "Zoe", points: 12, wins: 3 },
            Player { name: "Amy", points: 14, wins: 1 },
            Player { name: "Bob", points: 14, wins: 4 },
            Player { name: "Eli", points: 12, wins: 5 },
            Player { name: "Ada", points: 14, wins: 4 },
        ];

        let got = leaderboard(&players);
        let expected = vec![
            "1. Ada (14, 4)",
            "2. Bob (14, 4)",
            "3. Amy (14, 1)",
            "4. Eli (12, 5)",
            "5. Zoe (12, 3)",
        ];

        assert_eq!(got, expected);
    }

    #[test]
    fn stable_rank_numbers_after_sorting() {
        let players = vec![
            Player { name: "Ian", points: 9, wins: 2 },
            Player { name: "Gus", points: 9, wins: 2 },
            Player { name: "Nia", points: 7, wins: 8 },
        ];

        let got = leaderboard(&players);
        assert_eq!(got[0], "1. Gus (9, 2)");
        assert_eq!(got[1], "2. Ian (9, 2)");
        assert_eq!(got[2], "3. Nia (7, 8)");
    }

    #[test]
    fn empty_input_returns_empty_list() {
        let got = leaderboard(&[]);
        assert!(got.is_empty());
    }
}
