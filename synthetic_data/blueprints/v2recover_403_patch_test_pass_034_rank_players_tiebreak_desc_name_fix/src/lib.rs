#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player<'a> {
    pub name: &'a str,
    pub score: u32,
    pub penalties: u32,
}

pub fn leaderboard(players: &[Player<'_>]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.penalties.cmp(&b.penalties))
            .then_with(|| a.name.cmp(b.name))
    });
    items
        .into_iter()
        .map(|p| format!("{}:{}:{}", p.name, p.score, p.penalties))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_score_then_fewer_penalties_then_name_desc() {
        let players = vec![
            Player { name: "Nia", score: 12, penalties: 1 },
            Player { name: "Ava", score: 15, penalties: 2 },
            Player { name: "Zoe", score: 15, penalties: 2 },
            Player { name: "Ian", score: 15, penalties: 0 },
            Player { name: "Moe", score: 12, penalties: 0 },
        ];

        let got = leaderboard(&players);
        let want = vec![
            "Ian:15:0",
            "Zoe:15:2",
            "Ava:15:2",
            "Moe:12:0",
            "Nia:12:1",
        ];

        assert_eq!(got, want);
    }

    #[test]
    fn name_tiebreak_is_reverse_alphabetical_only_after_other_fields_tie() {
        let players = vec![
            Player { name: "Ada", score: 8, penalties: 1 },
            Player { name: "Bea", score: 8, penalties: 1 },
            Player { name: "Cal", score: 8, penalties: 0 },
            Player { name: "Dex", score: 7, penalties: 0 },
        ];

        let got = leaderboard(&players);
        let want = vec![
            "Cal:8:0",
            "Bea:8:1",
            "Ada:8:1",
            "Dex:7:0",
        ];

        assert_eq!(got, want);
    }
}
