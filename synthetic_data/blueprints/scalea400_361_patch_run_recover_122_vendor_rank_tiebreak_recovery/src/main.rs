use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Vendor {
    name: &'static str,
    points: u32,
    wins: u32,
    penalties: u32,
}

fn cmp_vendor(a: &Vendor, b: &Vendor) -> Ordering {
    a.points
        .cmp(&b.points)
        .then_with(|| a.penalties.cmp(&b.penalties))
        .then_with(|| a.wins.cmp(&b.wins))
        .then_with(|| b.name.cmp(a.name))
}

fn data() -> Vec<Vendor> {
    vec![
        Vendor { name: "Nova", points: 21, wins: 7, penalties: 9 },
        Vendor { name: "Apex", points: 21, wins: 7, penalties: 12 },
        Vendor { name: "Orbit", points: 21, wins: 6, penalties: 4 },
        Vendor { name: "Blaze", points: 19, wins: 7, penalties: 11 },
        Vendor { name: "Blaze", points: 18, wins: 9, penalties: 8 },
        Vendor { name: "Cinder", points: 19, wins: 7, penalties: 14 },
        Vendor { name: "Drift", points: 19, wins: 6, penalties: 3 },
        Vendor { name: "Ember", points: 18, wins: 8, penalties: 20 },
        Vendor { name: "Ember", points: 18, wins: 8, penalties: 25 },
    ]
}

fn render_ranking(mut vendors: Vec<Vendor>) -> String {
    vendors.sort_by(cmp_vendor);
    vendors
        .into_iter()
        .enumerate()
        .map(|(idx, v)| {
            format!(
                "{}. {} | pts={} wins={} pen={}",
                idx + 1,
                v.name,
                v.points,
                v.wins,
                v.penalties
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    println!("{}", render_ranking(data()));
}
