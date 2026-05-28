#[derive(Clone, Debug)]
pub struct Task {
    pub team: &'static str,
    pub points: u32,
    pub completed: bool,
    pub overdue: bool,
    pub archived_team: bool,
}

pub fn build_report(tasks: &[Task]) -> String {
    let mut rows: Vec<(&str, u32, u32)> = Vec::new();

    for task in tasks {
        let entry = rows.iter_mut().find(|(team, _, _)| *team == task.team);
        match entry {
            Some((_, total_points, overdue_count)) => {
                *total_points += task.points;
                if task.overdue {
                    *overdue_count += 1;
                }
            }
            None => rows.push((
                task.team,
                task.points,
                if task.overdue { 1 } else { 0 },
            )),
        }
    }

    rows.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::new();
    for (team, points, overdue) in rows {
        out.push_str(&format!("{}: {} pts ({} overdue)\n", team, points, overdue));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_only_active_completed_work() {
        let tasks = vec![
            Task { team: "beta", points: 5, completed: true, overdue: false, archived_team: false },
            Task { team: "alpha", points: 3, completed: false, overdue: true, archived_team: false },
            Task { team: "beta", points: 2, completed: true, overdue: true, archived_team: false },
            Task { team: "alpha", points: 7, completed: true, overdue: true, archived_team: false },
            Task { team: "ops", points: 4, completed: true, overdue: false, archived_team: true },
        ];

        let report = build_report(&tasks);
        assert_eq!(report, "alpha: 7 pts (1 overdue)\nbeta: 7 pts (1 overdue)\n");
    }

    #[test]
    fn omits_teams_with_no_completed_tasks() {
        let tasks = vec![
            Task { team: "gamma", points: 8, completed: false, overdue: true, archived_team: false },
            Task { team: "delta", points: 1, completed: true, overdue: false, archived_team: false },
        ];

        let report = build_report(&tasks);
        assert_eq!(report, "delta: 1 pts (0 overdue)\n");
    }
}
