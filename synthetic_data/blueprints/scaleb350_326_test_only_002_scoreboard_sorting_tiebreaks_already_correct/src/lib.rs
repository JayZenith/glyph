#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
}

impl Player {
    pub fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }

    pub fn games_played(&self) -> u32 {
        self.wins + self.draws + self.losses
    }
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.points()
            .cmp(&a.points())
            .then_with(|| b.wins.cmp(&a.wins))
            .then_with(|| a.losses.cmp(&b.losses))
            .then_with(|| a.games_played().cmp(&b.games_played()))
            .then_with(|| a.name.cmp(&b.name))
    });

    items
        .into_iter()
        .map(|p| format!("{}:{}", p.name, p.points()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, wins: u32, draws: u32, losses: u32) -> Player {
        Player {
            name: name.to_string(),
            wins,
            draws,
            losses,
        }
    }

    #[test]
    fn sorts_by_points_descending() {
        let players = vec![
            p("Cobra", 1, 0, 0),
            p("Atlas", 2, 0, 1),
            p("Blaze", 1, 1, 0),
        ];

        let got = leaderboard(&players);
        assert_eq!(got, vec!["Atlas:6", "Blaze:4", "Cobra:3"]);
    }

    #[test]
    fn breaks_point_ties_by_more_wins_then_fewer_losses() {
        let players = vec![
            p("Alpha", 2, 0, 2),
            p("Bravo", 1, 3, 0),
            p("Charlie", 2, 0, 1),
        ];

        let got = leaderboard(&players);
        assert_eq!(got, vec!["Charlie:6", "Alpha:6", "Bravo:6"]);
    }

    #[test]
    fn then_prefers_fewer_games_played() {
        let players = vec![
            p("Delta", 2, 0, 1),
            p("Echo", 2, 0, 2),
            p("Foxtrot", 2, 0, 1),
        ];

        let got = leaderboard(&players);
        assert_eq!(got, vec!["Delta:6", "Foxtrot:6", "Echo:6"]);
    }

    #[test]
    fn final_tiebreak_is_name_ascending() {
        let players = vec![
            p("Zed", 1, 2, 0),
            p("Amy", 1, 2, 0),
            p("Moe", 1, 2, 0),
        ];

        let got = leaderboard(&players);
        assert_eq!(got, vec!["Amy:5", "Moe:5", "Zed:5"]);
    }

    #[test]
    fn empty_input_returns_empty_list() {
        let got = leaderboard(&[]);
        assert!(got.is_empty());
    }
}
