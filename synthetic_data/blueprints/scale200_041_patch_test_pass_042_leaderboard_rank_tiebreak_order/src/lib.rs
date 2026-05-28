#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub points: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then(a.wins.cmp(&b.wins))
            .then(a.name.cmp(&b.name))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(idx, p)| format!("{}. {} ({} pts, {} wins)", idx + 1, p.name, p.points, p.wins))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, points: u32, wins: u32) -> Player {
        Player {
            name: name.to_string(),
            points,
            wins,
        }
    }

    #[test]
    fn ranks_by_points_then_wins_descending() {
        let players = vec![p("Ava", 12, 3), p("Ben", 15, 1), p("Cora", 15, 4)];
        let lines = leaderboard(&players);
        assert_eq!(lines[0], "1. Cora (15 pts, 4 wins)");
        assert_eq!(lines[1], "2. Ben (15 pts, 1 wins)");
        assert_eq!(lines[2], "3. Ava (12 pts, 3 wins)");
    }

    #[test]
    fn breaks_full_ties_by_name_ascending() {
        let players = vec![p("Zoe", 20, 5), p("Amy", 20, 5), p("Mia", 20, 5)];
        let lines = leaderboard(&players);
        assert_eq!(lines[0], "1. Amy (20 pts, 5 wins)");
        assert_eq!(lines[1], "2. Mia (20 pts, 5 wins)");
        assert_eq!(lines[2], "3. Zoe (20 pts, 5 wins)");
    }

    #[test]
    fn numbering_reflects_sorted_order() {
        let players = vec![p("Noah", 8, 2), p("Liam", 9, 0)];
        let lines = leaderboard(&players);
        assert_eq!(lines, vec!["1. Liam (9 pts, 0 wins)", "2. Noah (8 pts, 2 wins)"]);
    }
}
