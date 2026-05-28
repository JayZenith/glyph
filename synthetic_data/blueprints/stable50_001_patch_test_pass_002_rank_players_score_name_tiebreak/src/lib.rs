#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.wins.cmp(&b.wins))
            .then_with(|| b.name.cmp(&a.name))
    });

    items
        .into_iter()
        .map(|p| format!("{}:{}:{}", p.name, p.score, p.wins))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: u32, wins: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            wins,
        }
    }

    #[test]
    fn sorts_by_score_desc_then_wins_desc_then_name_asc() {
        let players = vec![
            p("Mia", 12, 4),
            p("Zoe", 15, 2),
            p("Ava", 15, 3),
            p("Liam", 15, 3),
            p("Noah", 12, 7),
        ];

        let got = leaderboard(&players);
        let want = vec![
            "Ava:15:3",
            "Liam:15:3",
            "Zoe:15:2",
            "Noah:12:7",
            "Mia:12:4",
        ];

        assert_eq!(got, want);
    }

    #[test]
    fn alphabetical_name_breaks_complete_ties() {
        let players = vec![
            p("Chris", 8, 1),
            p("Ben", 8, 1),
            p("Alex", 8, 1),
        ];

        let got = leaderboard(&players);
        let want = vec!["Alex:8:1", "Ben:8:1", "Chris:8:1"];

        assert_eq!(got, want);
    }
}
