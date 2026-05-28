#[derive(Clone, Debug)]
pub struct Member {
    pub team: &'static str,
    pub points: u32,
    pub active: bool,
}

pub fn team_report(teams: &[&'static str], members: &[Member]) -> Vec<String> {
    let mut totals: Vec<(&'static str, u32)> = Vec::new();

    for &team in teams {
        let mut sum = 0;
        let mut seen = false;
        for m in members {
            if m.team == team {
                seen = true;
                sum += m.points;
            }
        }
        if seen {
            totals.push((team, sum));
        }
    }

    totals.sort_by(|a, b| a.0.cmp(b.0));
    totals
        .into_iter()
        .map(|(team, total)| format!("{}:{}", team, total))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_zero_totals_and_sorts_by_total_desc_then_team() {
        let teams = ["blue", "red", "green"];
        let members = [
            Member { team: "red", points: 5, active: true },
            Member { team: "red", points: 7, active: false },
            Member { team: "blue", points: 4, active: true },
            Member { team: "blue", points: 6, active: true },
            Member { team: "green", points: 9, active: false },
        ];

        assert_eq!(
            team_report(&teams, &members),
            vec!["blue:10", "red:5", "green:0"]
        );
    }

    #[test]
    fn breaks_equal_totals_by_team_name() {
        let teams = ["ops", "api", "web"];
        let members = [
            Member { team: "ops", points: 3, active: true },
            Member { team: "api", points: 3, active: true },
            Member { team: "web", points: 1, active: true },
        ];

        assert_eq!(team_report(&teams, &members), vec!["api:3", "ops:3", "web:1"]);
    }
}
