#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: &'static str,
    pub score: u32,
    pub penalties: u32,
    pub submitted_at: u32,
}

pub fn leaderboard(mut entries: Vec<Entry>) -> Vec<&'static str> {
    entries.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.penalties.cmp(&b.penalties))
            .then_with(|| b.submitted_at.cmp(&a.submitted_at))
    });
    entries.into_iter().map(|e| e.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_score_then_penalties_then_earlier_submission() {
        let names = leaderboard(vec![
            Entry { name: "Kai", score: 120, penalties: 1, submitted_at: 30 },
            Entry { name: "Mia", score: 150, penalties: 4, submitted_at: 80 },
            Entry { name: "Noah", score: 150, penalties: 2, submitted_at: 70 },
            Entry { name: "Lia", score: 150, penalties: 2, submitted_at: 40 },
        ]);

        assert_eq!(names, vec!["Lia", "Noah", "Mia", "Kai"]);
    }

    #[test]
    fn alphabetical_name_breaks_full_ties() {
        let names = leaderboard(vec![
            Entry { name: "Zoe", score: 90, penalties: 3, submitted_at: 10 },
            Entry { name: "Amy", score: 90, penalties: 3, submitted_at: 10 },
            Entry { name: "Ben", score: 90, penalties: 3, submitted_at: 10 },
        ]);

        assert_eq!(names, vec!["Amy", "Ben", "Zoe"]);
    }
}
