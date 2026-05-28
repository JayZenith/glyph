fn parse_line(line: &str) -> Option<(String, String, u32)> {
    let mut id = None;
    let mut name = None;
    let mut score = None;

    for part in line.split(';') {
        let (k, v) = part.split_once('=')?;
        match k {
            "id" => id = Some(v.to_string()),
            "name" => name = Some(v.to_string()),
            "score" => score = v.parse::<u32>().ok(),
            _ => {}
        }
    }

    Some((id?, name.unwrap_or_default(), score.unwrap_or(0)))
}

fn main() {
    let input = [
        "id=u1;name=Ann;score=7",
        "id=u2;name=Bob;score=0",
        "id=u3;score=5",
        "id=u4;name=Dee;score=12",
        "id=u5;name=Eve;score=4;extra=x",
        "name=NoId;score=3",
        "id=u6;name=;score=9",
    ];

    let mut out = Vec::new();
    for line in input {
        if let Some((id, name, score)) = parse_line(line) {
            if score <= 10 {
                out.push(format!("{}:{}:{}", id, name, score));
            }
        }
    }

    println!("{}", out.join("\n"));
}
