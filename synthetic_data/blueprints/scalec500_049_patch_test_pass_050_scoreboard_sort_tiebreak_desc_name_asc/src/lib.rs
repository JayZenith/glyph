#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player<'a> {
    pub name: &'a str,
    pub wins: u32,
}

pub fn ranked_names(players: &[Player<'_>]) -> Vec<String> {
    let mut items = players.to_vec();
    items.sort_by(|a, b| a.wins.cmp(&b.wins).then_with(|| a.name.cmp(b.name)));
    items.into_iter().map(|p| p.name.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_wins_descending() {
        let players = [
            Player { name: "Rae", wins: 2 },
            Player { name: "Kai", wins: 5 },
            Player { name: "Mia", wins: 3 },
        ];
        assert_eq!(ranked_names(&players), vec!["Kai", "Mia", "Rae"]);
    }

    #[test]
    fn breaks_ties_by_name_ascending() {
        let players = [
            Player { name: "Zoe", wins: 4 },
            Player { name: "Ana", wins: 4 },
            Player { name: "Ben", wins: 4 },
        ];
        assert_eq!(ranked_names(&players), vec!["Ana", "Ben", "Zoe"]);
    }

    #[test]
    fn mixed_scores_and_ties() {
        let players = [
            Player { name: "Lia", wins: 1 },
            Player { name: "Eli", wins: 3 },
            Player { name: "Ava", wins: 3 },
            Player { name: "Noa", wins: 2 },
        ];
        assert_eq!(ranked_names(&players), vec!["Ava", "Eli", "Noa", "Lia"]);
    }
}
