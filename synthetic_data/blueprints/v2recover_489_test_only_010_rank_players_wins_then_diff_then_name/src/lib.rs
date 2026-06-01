use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub wins: u32,
    pub losses: u32,
}

impl Player {
    pub fn new(name: &str, wins: u32, losses: u32) -> Self {
        Self {
            name: name.to_string(),
            wins,
            losses,
        }
    }

    fn diff(&self) -> i32 {
        self.wins as i32 - self.losses as i32
    }
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut ranked = players.to_vec();
    ranked.sort_by_key(|p| (Reverse(p.wins), Reverse(p.diff()), p.name.clone()));
    ranked
        .into_iter()
        .map(|p| format!("{} ({}-{})", p.name, p.wins, p.losses))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_wins_desc_then_diff_desc_then_name_asc() {
        let players = vec![
            Player::new("Mira", 7, 2),
            Player::new("Ava", 8, 4),
            Player::new("Zed", 8, 4),
            Player::new("Noel", 8, 3),
        ];

        let got = leaderboard(&players);
        let expected = vec![
            "Noel (8-3)",
            "Ava (8-4)",
            "Zed (8-4)",
            "Mira (7-2)",
        ];

        assert_eq!(got, expected);
    }

    #[test]
    fn keeps_duplicate_records_but_orders_names_for_exact_ties() {
        let players = vec![
            Player::new("Liam", 5, 5),
            Player::new("Emma", 5, 5),
            Player::new("Olive", 4, 1),
            Player::new("Mason", 5, 5),
        ];

        let got = leaderboard(&players);
        let expected = vec![
            "Emma (5-5)",
            "Liam (5-5)",
            "Mason (5-5)",
            "Olive (4-1)",
        ];

        assert_eq!(got, expected);
    }

    #[test]
    fn empty_input_produces_empty_leaderboard() {
        let players = vec![];
        let got = leaderboard(&players);
        assert!(got.is_empty());
    }
}
