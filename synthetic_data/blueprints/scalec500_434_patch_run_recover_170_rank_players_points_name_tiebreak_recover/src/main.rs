#[derive(Clone, Copy)]
struct Player {
    name: &'static str,
    points: u32,
    wins: u32,
}

fn render(players: &mut Vec<Player>) -> String {
    players.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| a.name.cmp(&b.name))
    });

    let mut out = String::new();
    let mut prev_points = None;
    let mut rank = 0usize;

    for (idx, p) in players.iter().enumerate() {
        if prev_points != Some(p.points) {
            rank = idx + 1;
            prev_points = Some(p.points);
        }

        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(&format!("{}. {} ({} pts, {} wins)", rank, p.name, p.points, p.wins));
    }

    out
}

fn main() {
    let mut players = vec![
        Player {
            name: "Bo",
            points: 12,
            wins: 5,
        },
        Player {
            name: "Dia",
            points: 9,
            wins: 6,
        },
        Player {
            name: "Cy",
            points: 9,
            wins: 3,
        },
        Player {
            name: "Ava",
            points: 12,
            wins: 5,
        },
        Player {
            name: "Eli",
            points: 12,
            wins: 4,
        },
    ];

    print!("{}", render(&mut players));
}
