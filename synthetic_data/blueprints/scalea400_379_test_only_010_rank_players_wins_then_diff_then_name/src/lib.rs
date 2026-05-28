use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub wins: u32,
    pub losses: u32,
}

impl Player {
    pub fn diff(&self) -> i32 {
        self.wins as i32 - self.losses as i32
    }
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by_key(|p| (Reverse(p.wins), Reverse(p.diff()), p.name.clone()));
    rows.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, wins: u32, losses: u32) -> Player {
        Player {
            name: name.to_string(),
            wins,
            losses,
        }
    }

    #[test]
    fn sorts_by_wins_desc_then_diff_desc_then_name_asc() {
        let players = vec![
            p("Zoe", 8, 3),
            p("Amy", 9, 5),
            p("Ben", 9, 5),
            p("Cara", 9, 4),
            p("Dan", 8, 1),
        ];

        assert_eq!(
            leaderboard(&players),
            vec!["Cara", "Amy", "Ben", "Dan", "Zoe"]
        );
    }

    #[test]
    fn tie_on_wins_can_be_broken_by_negative_diff() {
        let players = vec![
            p("Kai", 4, 6),
            p("Lia", 4, 2),
            p("Moe", 4, 2),
            p("Ava", 5, 9),
        ];

        assert_eq!(leaderboard(&players), vec!["Ava", "Lia", "Moe", "Kai"]);
    }

    #[test]
    fn empty_input_returns_empty_board() {
        let players: Vec<Player> = Vec::new();
        assert!(leaderboard(&players).is_empty());
    }
}
