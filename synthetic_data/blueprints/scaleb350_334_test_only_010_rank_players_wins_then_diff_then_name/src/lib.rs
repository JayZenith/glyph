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

pub fn leaderboard(players: &[Player]) -> Vec<&'static str> {
    let mut refs: Vec<&Player> = players.iter().collect();
    refs.sort_by(|a, b| {
        b.wins
            .cmp(&a.wins)
            .then_with(|| b.diff().cmp(&a.diff()))
            .then_with(|| a.name.cmp(b.name))
    });
    refs.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks_by_wins_descending() {
        let players = [
            Player { name: "Kai", wins: 7, losses: 4 },
            Player { name: "Mia", wins: 10, losses: 9 },
            Player { name: "Zoe", wins: 8, losses: 1 },
        ];
        assert_eq!(leaderboard(&players), vec!["Mia", "Zoe", "Kai"]);
    }

    #[test]
    fn breaks_win_ties_by_better_differential() {
        let players = [
            Player { name: "Ava", wins: 6, losses: 1 },
            Player { name: "Ben", wins: 6, losses: 3 },
            Player { name: "Cole", wins: 5, losses: 0 },
        ];
        assert_eq!(leaderboard(&players), vec!["Ava", "Ben", "Cole"]);
    }

    #[test]
    fn breaks_full_ties_alphabetically() {
        let players = [
            Player { name: "Liam", wins: 4, losses: 2 },
            Player { name: "Noah", wins: 4, losses: 2 },
            Player { name: "Emma", wins: 4, losses: 2 },
        ];
        assert_eq!(leaderboard(&players), vec!["Emma", "Liam", "Noah"]);
    }

    #[test]
    fn handles_empty_input() {
        let players: [Player; 0] = [];
        assert!(leaderboard(&players).is_empty());
    }

    #[test]
    fn combines_all_tiebreak_rules() {
        let players = [
            Player { name: "Ivy", wins: 9, losses: 4 },
            Player { name: "Aria", wins: 9, losses: 4 },
            Player { name: "Omar", wins: 9, losses: 2 },
            Player { name: "Bea", wins: 8, losses: 0 },
            Player { name: "Dax", wins: 8, losses: 3 },
        ];
        assert_eq!(leaderboard(&players), vec!["Omar", "Aria", "Ivy", "Bea", "Dax"]);
    }
}
