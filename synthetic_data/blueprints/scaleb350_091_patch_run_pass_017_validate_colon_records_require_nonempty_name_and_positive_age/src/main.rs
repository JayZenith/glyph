fn parse_valid_names(input: &str) -> Vec<String> {
    input
        .lines()
        .filter_map(|line| {
            let (name, age_text) = line.split_once(':')?;
            let age: u32 = age_text.parse().ok()?;
            if age >= 0 {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let data = "Alice:30\nBob:0\n:22\nCara:19\nDylan:abc\nEve-42";
    let names = parse_valid_names(data);
    let total = data.lines().count();
    let bad = total - names.len();

    println!("ok:{} bad:{}", names.len(), bad);
    for name in names {
        println!("{}", name);
    }
}
