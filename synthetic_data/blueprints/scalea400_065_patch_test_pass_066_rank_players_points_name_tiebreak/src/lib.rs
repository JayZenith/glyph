#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub points: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| a.wins.cmp(&b.wins))
            .then_with(|| b.name.cmp(&a.name))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} ({} pts, {} wins)", i + 1, p.name, p.points, p.wins))
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
    fn sorts_by_points_then_wins_then_name() {
        let players = vec![
            p("Zoe", 12, 4),
            p("Ana", 12, 5),
            p("Bob", 12, 5),
            p("Mia", 9, 7),
        ];

        let lines = leaderboard(&players);
        let names: Vec<&str> = lines
            .iter()
            .map(|s| s.split_once('.').unwrap().1.trim())
            .map(|s| s.split_once(" (").unwrap().0)
            .collect();

        assert_eq!(names, vec!["Ana", "Bob", "Zoe", "Mia"]);
    }

    #[test]
    fn numbering_follows_sorted_order() {
        let players = vec![p("Cara", 3, 1), p("Bea", 8, 0), p("Ava", 8, 2)];
        let lines = leaderboard(&players);

        assert_eq!(lines[0], "1. Ava (8 pts, 2 wins)");
        assert_eq!(lines[1], "2. Bea (8 pts, 0 wins)");
        assert_eq!(lines[2], "3. Cara (3 pts, 1 wins)");
    }
}
