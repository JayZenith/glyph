fn parse_line(line: &str) -> Option<(u32, String, u32)> {
    let mut id = None;
    let mut name = None;
    let mut age = None;

    for part in line.split(';') {
        let (k, v) = part.split_once('=')?;
        match k {
            "id" => id = v.parse::<u32>().ok(),
            "name" => name = Some(v.to_string()),
            "age" => age = v.parse::<u32>().ok(),
            _ => {}
        }
    }

    Some((id?, name?, age?))
}

fn valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_ascii_alphabetic())
}

fn main() {
    let input = "id=1;name=alice;age=30\nname=bob;id=2;age=0\nid=3;name=carol;age=27;extra=x\nid=4;name=dave;age=44\nid=5;name=eve2;age=22";

    let mut out = Vec::new();
    for line in input.lines() {
        if let Some((id, name, age)) = parse_line(line) {
            if valid_name(&name) {
                out.push(format!("ok:{}:{}", id, name));
            }
        }
    }

    print!("{}", out.join("\n"));
}
