#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub rounds: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.rounds.cmp(&b.rounds))
            .then_with(|| b.name.cmp(&a.name))
    });

    items
        .into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} [{} in {} rounds]", i + 1, p.name, p.score, p.rounds))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: u32, rounds: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            rounds,
        }
    }

    #[test]
    fn sorts_by_score_then_fewer_rounds_then_name() {
        let players = vec![
            p("Zoe", 15, 4),
            p("Amy", 20, 5),
            p("Bob", 20, 3),
            p("Cara", 20, 3),
            p("Dan", 15, 2),
        ];

        let lines = leaderboard(&players);
        assert_eq!(
            lines,
            vec![
                "1. Bob [20 in 3 rounds]",
                "2. Cara [20 in 3 rounds]",
                "3. Amy [20 in 5 rounds]",
                "4. Dan [15 in 2 rounds]",
                "5. Zoe [15 in 4 rounds]",
            ]
        );
    }

    #[test]
    fn stable_format_for_single_entry() {
        let lines = leaderboard(&[p("Mia", 7, 1)]);
        assert_eq!(lines, vec!["1. Mia [7 in 1 rounds]"]);
    }

    #[test]
    fn empty_input_returns_empty_output() {
        let lines = leaderboard(&[]);
        assert!(lines.is_empty());
    }
}
