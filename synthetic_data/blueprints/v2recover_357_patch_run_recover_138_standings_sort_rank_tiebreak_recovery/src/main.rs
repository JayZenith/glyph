use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Team {
    name: &'static str,
    pts: u32,
    gf: i32,
    ga: i32,
}

impl Team {
    fn gd(&self) -> i32 {
        self.gf - self.ga
    }
}

fn rank_key_cmp(a: &Team, b: &Team) -> Ordering {
    a.pts.cmp(&b.pts)
        .then(a.gd().cmp(&b.gd()))
        .then(a.name.cmp(b.name))
}

fn same_rank(a: &Team, b: &Team) -> bool {
    a.pts == b.pts && a.gd() == b.gd()
}

fn format_table(teams: &mut Vec<Team>) -> String {
    teams.sort_by(rank_key_cmp);
    let mut lines = Vec::new();
    let mut shown_rank = 1usize;

    for i in 0..teams.len() {
        if i > 0 && !same_rank(&teams[i - 1], &teams[i]) {
            shown_rank += 1;
        }
        let t = &teams[i];
        lines.push(format!(
            "{}. {:<7} {} pts GD {:+} GS {}",
            shown_rank,
            t.name,
            t.pts,
            t.gd(),
            t.gf
        ));
    }

    lines.join("\n")
}

fn main() {
    let mut teams = vec![
        Team { name: "Cobras", pts: 7, gf: 5, ga: 2 },
        Team { name: "Falcons", pts: 4, gf: 3, ga: 3 },
        Team { name: "Lynx", pts: 4, gf: 4, ga: 4 },
        Team { name: "Owls", pts: 0, gf: 1, ga: 4 },
        Team { name: "Bears", pts: 4, gf: 4, ga: 4 },
    ];

    println!("{}", format_table(&mut teams));
}
