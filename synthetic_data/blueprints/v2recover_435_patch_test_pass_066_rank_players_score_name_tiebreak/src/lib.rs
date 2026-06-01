use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub wins: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut best_by_name: HashMap<String, Player> = HashMap::new();

    for p in players {
        match best_by_name.get(&p.name) {
            Some(existing) if existing.score >= p.score => {}
            _ => {
                best_by_name.insert(p.name.clone(), p.clone());
            }
        }
    }

    let mut entries: Vec<Player> = best_by_name.into_values().collect();
    entries.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .then(a.wins.cmp(&b.wins))
            .then(a.name.cmp(&b.name))
    });

    entries
        .into_iter()
        .enumerate()
        .map(|(i, p)| format!("{}. {} ({}, {} wins)", i + 1, p.name, p.score, p.wins))
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
    fn sorts_by_score_then_wins_then_name() {
        let players = vec![
            p("Zoe", 10, 2),
            p("Amy", 15, 1),
            p("Ben", 15, 3),
            p("Ada", 15, 3),
        ];

        let got = leaderboard(&players);
        let want = vec![
            "1. Ada (15, 3 wins)",
            "2. Ben (15, 3 wins)",
            "3. Amy (15, 1 wins)",
            "4. Zoe (10, 2 wins)",
        ];

        assert_eq!(got, want);
    }

    #[test]
    fn keeps_best_duplicate_entry_using_wins_as_tiebreaker() {
        let players = vec![
            p("Kai", 12, 1),
            p("Mia", 9, 4),
            p("Kai", 12, 5),
            p("Mia", 11, 1),
        ];

        let got = leaderboard(&players);
        let want = vec![
            "1. Kai (12, 5 wins)",
            "2. Mia (11, 1 wins)",
        ];

        assert_eq!(got, want);
    }
}
