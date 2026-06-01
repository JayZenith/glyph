#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub points: u32,
    pub wins: u32,
    pub goal_diff: i32,
    pub goals_scored: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.wins.cmp(&b.wins))
            .then(a.goal_diff.cmp(&b.goal_diff))
            .then(a.goals_scored.cmp(&b.goals_scored))
            .then(a.name.cmp(b.name))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} [{}p|{}w|gd{}|{}gs]", i + 1, p.name, p.points, p.wins, p.goal_diff, p.goals_scored))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Player> {
        vec![
            Player { name: "Lions", points: 12, wins: 4, goal_diff: 5, goals_scored: 9 },
            Player { name: "Bears", points: 12, wins: 4, goal_diff: 5, goals_scored: 11 },
            Player { name: "Hawks", points: 12, wins: 4, goal_diff: 7, goals_scored: 8 },
            Player { name: "Owls", points: 10, wins: 3, goal_diff: 2, goals_scored: 10 },
            Player { name: "Ants", points: 10, wins: 3, goal_diff: 2, goals_scored: 10 },
            Player { name: "Moles", points: 10, wins: 2, goal_diff: 9, goals_scored: 12 },
        ]
    }

    #[test]
    fn sorts_by_all_tiebreakers() {
        let board = leaderboard(&sample());
        let names: Vec<&str> = board
            .iter()
            .map(|line| line.split_once('.').unwrap().1.trim())
            .map(|rest| rest.split_once(" [").unwrap().0)
            .collect();

        assert_eq!(names, vec!["Hawks", "Bears", "Lions", "Ants", "Owls", "Moles"]);
    }

    #[test]
    fn highest_rank_is_number_one() {
        let board = leaderboard(&sample());
        assert!(board[0].starts_with("1. Hawks "));
        assert!(board[1].starts_with("2. Bears "));
        assert!(board[5].starts_with("6. Moles "));
    }

    #[test]
    fn alphabetical_name_breaks_full_stat_ties() {
        let board = leaderboard(&sample());
        let ants = board.iter().position(|line| line.contains("Ants")).unwrap();
        let owls = board.iter().position(|line| line.contains("Owls")).unwrap();
        assert!(ants < owls);
    }

    #[test]
    fn format_shows_sign_for_goal_diff() {
        let board = leaderboard(&[
            Player { name: "Reds", points: 1, wins: 0, goal_diff: -3, goals_scored: 2 },
            Player { name: "Blues", points: 2, wins: 0, goal_diff: 0, goals_scored: 1 },
        ]);
        assert_eq!(board[0], "1. Blues [2p|0w|gd+0|1gs]");
        assert_eq!(board[1], "2. Reds [1p|0w|gd-3|2gs]");
    }

    #[test]
    fn empty_input_returns_empty_board() {
        let board = leaderboard(&[]);
        assert!(board.is_empty());
    }
}
