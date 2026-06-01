use std::cmp::Ordering;

#[derive(Clone, Copy)]
struct Team {
    name: &'static str,
    points: u32,
    gd: i32,
    gs: u32,
}

fn best_rows(rows: &[Team]) -> Vec<Team> {
    let mut out: Vec<Team> = Vec::new();
    for row in rows {
        if let Some(existing) = out.iter_mut().find(|t| t.name == row.name) {
            if row.points > existing.points {
                *existing = *row;
            }
        } else {
            out.push(*row);
        }
    }
    out
}

fn main() {
    let rows = [
        Team { name: "Falcons", points: 12, gd: 5, gs: 10 },
        Team { name: "Bears", points: 12, gd: 7, gs: 9 },
        Team { name: "Cobras", points: 12, gd: 7, gs: 8 },
        Team { name: "Hawks", points: 12, gd: 7, gs: 8 },
        Team { name: "Lynx", points: 10, gd: 6, gs: 7 },
        Team { name: "Owls", points: 10, gd: 4, gs: 6 },
        Team { name: "Owls", points: 9, gd: 5, gs: 7 },
        Team { name: "Vipers", points: 8, gd: 1, gs: 5 },
    ];

    let mut teams = best_rows(&rows);
    teams.sort_by(|a, b| {
        a.points.cmp(&b.points)
            .then(a.gd.cmp(&b.gd))
            .then(a.gs.cmp(&b.gs))
            .then_with(|| b.name.cmp(a.name))
    });

    for (i, t) in teams.iter().enumerate() {
        println!("{}. {} {} pts gd {:+} gs {}", i + 1, t.name, t.points, t.gd, t.gs);
    }
}
