#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub wins: u32,
    pub losses: u32,
}

impl Player {
    fn diff(&self) -> i32 {
        self.wins as i32 - self.losses as i32
    }
}

pub fn leaderboard_names(players: &[Player]) -> Vec<&'static str> {
    let mut rows = players.to_vec();
    rows.sort_by(|a, b| {
        b.wins
            .cmp(&a.wins)
            .then_with(|| b.diff().cmp(&a.diff()))
            .then_with(|| a.name.cmp(b.name))
    });
    rows.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_wins_desc_then_diff_desc_then_name_asc() {
        let players = vec![
            Player {
                name: "Kai",
                wins: 7,
                losses: 3,
            },
            Player {
                name: "Ava",
                wins: 9,
                losses: 5,
            },
            Player {
                name: "Mia",
                wins: 9,
                losses: 4,
            },
            Player {
                name: "Ben",
                wins: 7,
                losses: 3,
            },
            Player {
                name: "Zoe",
                wins: 9,
                losses: 4,
            },
        ];

        assert_eq!(leaderboard_names(&players), vec!["Mia", "Zoe", "Ava", "Ben", "Kai"]);
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let players: Vec<Player> = Vec::new();
        assert!(leaderboard_names(&players).is_empty());
    }

    #[test]
    fn keeps_duplicate_records_when_all_fields_match() {
        let players = vec![
            Player {
                name: "Eli",
                wins: 5,
                losses: 5,
            },
            Player {
                name: "Eli",
                wins: 5,
                losses: 5,
            },
            Player {
                name: "Noa",
                wins: 5,
                losses: 6,
            },
        ];

        assert_eq!(leaderboard_names(&players), vec!["Eli", "Eli", "Noa"]);
    }
}
