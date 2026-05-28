const INPUT: &str = "A12|active|5|blue,green\nB9|active|3|red\nC77|paused|0|teal\nD44|retired|4|silver\nE11|active|2|yellow\nF08|paused|7|orange,purple\nG10|active|x|black\nH22|paused|4|\nI33|active|4|navy,cyan,magenta\nJ55|active|8|amber";

#[derive(Debug)]
struct Record {
    id: String,
    status: String,
    score: u32,
    tags: Vec<String>,
}

fn parse_line(line: &str) -> Option<Record> {
    let mut parts = line.split('|');
    let id = parts.next()?.to_string();
    let status = parts.next()?.to_string();
    let score = parts.next()?.parse::<u32>().ok()?;
    let tags = parts
        .next()?
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    if parts.next().is_some() {
        return None;
    }

    if id.len() < 2 {
        return None;
    }

    if status != "active" && status != "paused" {
        return None;
    }

    if tags.is_empty() {
        return None;
    }

    Some(Record { id, status, score, tags })
}

fn main() {
    let mut records = INPUT.lines().filter_map(parse_line).collect::<Vec<_>>();
    records.retain(|r| r.score >= 2);

    println!("VALID {}", records.len());
    for r in records {
        println!("{}|{}|{}|{}", r.id, r.status, r.score, r.tags.join(","));
    }
}
