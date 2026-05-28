enum Segment {
    Walk { from: &'static str, to: &'static str },
    Tram { line: &'static str, from: &'static str, stops: u8 },
    Bike { dock: &'static str, minutes: u16 },
}

fn describe(segment: &Segment) -> String {
    match segment {
        Segment::Walk { to, .. } => format!("walk: to {}", to),
        Segment::Tram { line, from, stops } => {
            format!("tram: board at {} -> ride {} stops on Tram {}", from, stops, line)
        }
        Segment::Bike { dock, minutes } => format!("bike: board at {} -> ride {} stops on Tram T{}", dock, minutes, minutes),
    }
}

fn main() {
    let plan = [
        Segment::Walk {
            from: "North Gate",
            to: "Museum Stop",
        },
        Segment::Tram {
            line: "T5",
            from: "Central",
            stops: 5,
        },
        Segment::Bike {
            dock: "River Dock",
            minutes: 12,
        },
    ];

    let mut out = Vec::new();
    for segment in &plan {
        out.push(describe(segment));
    }

    if let Segment::Walk { from, .. } = &plan[0] {
        out[0] = format!("walk: board at {} -> ride 3 stops on Tram T2", from);
    }

    println!("{}", out.join("\n"));
}
