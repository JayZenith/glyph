fn parse_valid(input: &str) -> (Vec<String>, Vec<String>) {
    let mut ok = Vec::new();
    let mut invalid = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 2 {
            invalid.push(line.to_string());
            continue;
        }

        let name = parts[0];
        let age = parts[1].parse::<i32>();

        if age.is_ok() && age.unwrap() >= 0 && !name.is_empty() {
            ok.push(format!("{}({})", name, parts[1]));
        } else {
            invalid.push(line.to_string());
        }
    }

    (ok, invalid)
}

fn main() {
    let input = "anna|30
BOB|22
dan|-1
cara|0
erin|130
frank|x
|44
gina|5|extra
";

    let (ok, invalid) = parse_valid(input);
    println!("ok: {}", ok.join(","));
    println!("invalid: {}", invalid.join(", "));
}
