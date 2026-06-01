#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Team {
    pub name: &'static str,
    pub points: u32,
    pub goal_diff: i32,
    pub goals_scored: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedTeam {
    pub rank: usize,
    pub name: &'static str,
    pub points: u32,
    pub goal_diff: i32,
    pub goals_scored: u32,
}

pub fn rank_teams(teams: &[Team]) -> Vec<RankedTeam> {
    let mut teams = teams.to_vec();
    teams.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| a.name.cmp(b.name))
    });

    teams
        .into_iter()
        .enumerate()
        .map(|(idx, team)| RankedTeam {
            rank: idx + 1,
            name: team.name,
            points: team.points,
            goal_diff: team.goal_diff,
            goals_scored: team.goals_scored,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_all_tiebreakers() {
        let ranked = rank_teams(&[
            Team { name: "Bears", points: 10, goal_diff: 5, goals_scored: 12 },
            Team { name: "Arrows", points: 10, goal_diff: 7, goals_scored: 9 },
            Team { name: "Comets", points: 10, goal_diff: 7, goals_scored: 11 },
            Team { name: "Dragons", points: 10, goal_diff: 7, goals_scored: 11 },
            Team { name: "Eagles", points: 8, goal_diff: 10, goals_scored: 20 },
        ]);

        let names: Vec<_> = ranked.iter().map(|t| t.name).collect();
        assert_eq!(names, vec!["Comets", "Dragons", "Arrows", "Bears", "Eagles"]);
    }

    #[test]
    fn tied_teams_share_competition_rank() {
        let ranked = rank_teams(&[
            Team { name: "Alpha", points: 9, goal_diff: 3, goals_scored: 7 },
            Team { name: "Beta", points: 9, goal_diff: 3, goals_scored: 7 },
            Team { name: "Gamma", points: 7, goal_diff: 1, goals_scored: 4 },
            Team { name: "Delta", points: 6, goal_diff: 0, goals_scored: 5 },
        ]);

        let pairs: Vec<_> = ranked.iter().map(|t| (t.name, t.rank)).collect();
        assert_eq!(pairs, vec![("Alpha", 1), ("Beta", 1), ("Gamma", 3), ("Delta", 4)]);
    }

    #[test]
    fn empty_input_returns_empty_ranking() {
        let ranked = rank_teams(&[]);
        assert!(ranked.is_empty());
    }

    #[test]
    fn later_tie_after_unique_leader_skips_rank() {
        let ranked = rank_teams(&[
            Team { name: "Lions", points: 12, goal_diff: 8, goals_scored: 15 },
            Team { name: "Owls", points: 9, goal_diff: 2, goals_scored: 8 },
            Team { name: "Pandas", points: 9, goal_diff: 2, goals_scored: 8 },
            Team { name: "Quakes", points: 4, goal_diff: -1, goals_scored: 3 },
        ]);

        let pairs: Vec<_> = ranked.iter().map(|t| (t.name, t.rank)).collect();
        assert_eq!(pairs, vec![("Lions", 1), ("Owls", 2), ("Pandas", 2), ("Quakes", 4)]);
    }
}
