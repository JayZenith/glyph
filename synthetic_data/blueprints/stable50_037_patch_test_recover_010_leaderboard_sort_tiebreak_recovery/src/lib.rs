#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: &'static str,
    pub score: u32,
    pub wins: u32,
    pub losses: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut rows = players.to_vec();
    rows.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .then_with(|| a.wins.cmp(&b.wins))
            .then_with(|| a.name.cmp(b.name))
    });

    rows.into_iter()
        .map(|p| format!("{}:{}-{} ({})", p.name, p.wins, p.losses, p.score))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &'static str, score: u32, wins: u32, losses: u32) -> Player {
        Player { name, score, wins, losses }
    }

    #[test]
    fn sorts_by_score_then_wins_then_fewer_losses_then_name() {
        let players = vec![
            p("Zed", 30, 8, 2),
            p("Amy", 50, 5, 1),
            p("Bob", 50, 5, 3),
            p("Cara", 50, 7, 4),
            p("Dan", 30, 9, 0),
            p("Eli", 50, 7, 2),
        ];

        let got = leaderboard(&players);
        let want = vec![
            "Eli:7-2 (50)",
            "Cara:7-4 (50)",
            "Amy:5-1 (50)",
            "Bob:5-3 (50)",
            "Dan:9-0 (30)",
            "Zed:8-2 (30)",
        ];

        assert_eq!(got, want);
    }

    #[test]
    fn alphabetical_name_is_final_tiebreak_only() {
        let players = vec![
            p("Moe", 40, 6, 2),
            p("Ava", 40, 6, 2),
            p("Lia", 40, 6, 2),
        ];

        let got = leaderboard(&players);
        let want = vec![
            "Ava:6-2 (40)",
            "Lia:6-2 (40)",
            "Moe:6-2 (40)",
        ];

        assert_eq!(got, want);
    }
}
