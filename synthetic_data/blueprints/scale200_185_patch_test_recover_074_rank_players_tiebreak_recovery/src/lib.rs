use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player<'a> {
    pub name: &'a str,
    pub points: u32,
    pub last_win_round: u32,
}

pub fn leaderboard_names(players: &[Player<'_>]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then_with(|| a.last_win_round.cmp(&b.last_win_round))
            .then_with(|| a.name.cmp(b.name))
    });
    items.into_iter().map(|p| p.name.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_by_points_descending() {
        let players = [
            Player { name: "Kai", points: 12, last_win_round: 4 },
            Player { name: "Mia", points: 18, last_win_round: 7 },
            Player { name: "Zoe", points: 15, last_win_round: 3 },
        ];
        assert_eq!(leaderboard_names(&players), vec!["Mia", "Zoe", "Kai"]);
    }

    #[test]
    fn ties_use_earlier_last_win_round() {
        let players = [
            Player { name: "Ava", points: 20, last_win_round: 5 },
            Player { name: "Ben", points: 20, last_win_round: 2 },
            Player { name: "Cara", points: 20, last_win_round: 4 },
        ];
        assert_eq!(leaderboard_names(&players), vec!["Ben", "Cara", "Ava"]);
    }

    #[test]
    fn exact_ties_fall_back_to_name_descending() {
        let players = [
            Player { name: "Amy", points: 9, last_win_round: 6 },
            Player { name: "Zed", points: 9, last_win_round: 6 },
            Player { name: "Ian", points: 9, last_win_round: 6 },
        ];
        assert_eq!(leaderboard_names(&players), vec!["Zed", "Ian", "Amy"]);
    }

    #[test]
    fn mixed_rules_apply_together() {
        let players = [
            Player { name: "Liu", points: 14, last_win_round: 8 },
            Player { name: "Noa", points: 17, last_win_round: 9 },
            Player { name: "Eli", points: 17, last_win_round: 3 },
            Player { name: "Pia", points: 14, last_win_round: 8 },
        ];
        assert_eq!(leaderboard_names(&players), vec!["Eli", "Noa", "Pia", "Liu"]);
    }
}
