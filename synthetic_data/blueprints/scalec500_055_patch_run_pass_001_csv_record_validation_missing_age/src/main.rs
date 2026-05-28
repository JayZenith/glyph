fn valid_record(line: &str) -> bool {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 3 {
        return false;
    }

    let name = parts[0];
    let age = parts[1];
    let active = parts[2];

    if name.is_empty() {
        return false;
    }

    if age.parse::<u8>().is_err() {
        return false;
    }

    matches!(active, "true" | "false")
}

fn main() {
    let input = [
        "alice,30,true",
        "bob,,false",
        "carol,27,yes",
        ",22,true",
        "dave,41,false",
    ];

    let ok = input.iter().filter(|line| valid_record(line)).count();
    let err = input.len() - ok;

    println!("ok:{} err:{}", ok, err);
}
