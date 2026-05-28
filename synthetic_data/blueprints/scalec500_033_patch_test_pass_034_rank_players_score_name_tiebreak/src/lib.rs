#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub penalties: u32,
}

pub fn leaderboard(players: &[Player]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .then(a.penalties.cmp(&b.penalties))
            .then(a.name.cmp(&b.name))
    });
    items.into_iter().map(|p| p.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(name: &str, score: u32, penalties: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            penalties,
        }
    }

    #[test]
    fn sorts_by_score_desc_penalties_asc_name_asc() {
        let players = vec![
            p("zoe", 15, 2),
            p("amy", 20, 1),
            p("bob", 20, 0),
            p("cara", 15, 1),
            p("dave", 20, 0),
        ];

        let names = leaderboard(&players);
        assert_eq!(names, vec!["bob", "dave", "amy", "cara", "zoe"]);
    }

    #[test]
    fn keeps_alphabetical_order_when_all_numeric_fields_tie() {
        let players = vec![p("mila", 8, 2), p("alex", 8, 2), p("noah", 8, 2)];
        let names = leaderboard(&players);
        assert_eq!(names, vec!["alex", "mila", "noah"]);
    }
}
