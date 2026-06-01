const INPUT: &str = "id=1|name=Ava|score=90|status=ACTIVE\nid=2|name=Ben|score=77|status=INACTIVE\nid=3|name=Cam|score=abc|status=ACTIVE\nid=4|name=Dee|score=0|status=ACTIVE\nid=5|name=Eli|score=100|status=ACTIVE\nid=6|name=Fox|score=101|status=ACTIVE\nname=Gia|status=ACTIVE\nid=8|name=Hal|score=44|state=ACTIVE\nid=9|name=Ian|score=20|status=active";

fn parse_line(line: &str) -> Option<(String, i32, String)> {
    let mut id = false;
    let mut name = None;
    let mut score = None;
    let mut status = None;

    for field in line.split('|') {
        let mut parts = field.split('=');
        let key = parts.next()?;
        let value = parts.next()?;
        match key {
            "id" => id = true,
            "name" => name = Some(value.to_string()),
            "score" => score = value.parse::<i32>().ok(),
            "status" => status = Some(value.to_string()),
            _ => {}
        }
    }

    if id && name.is_some() && score.is_some() {
        Some((name?, score?, status.unwrap_or_default()))
    } else {
        None
    }
}

fn main() {
    let mut rows = Vec::new();
    for line in INPUT.lines() {
        if let Some((name, score, status)) = parse_line(line) {
            if status.to_uppercase() == "ACTIVE" {
                rows.push((name, score));
            }
        }
    }

    println!("valid={}", rows.len());
    for (name, score) in rows {
        println!("{}:{}", name, score);
    }
}
