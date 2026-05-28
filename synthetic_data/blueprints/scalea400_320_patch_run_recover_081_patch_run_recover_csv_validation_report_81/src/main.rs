const INPUT: &str = "A1,10,ok\nB2,-3,ok\nC3,7,warn\nD4,5,bad\nE5,12\nF6,abc,ok\n";

fn main() {
    let mut valid = 0;
    let mut invalid = 0;
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for line in INPUT.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            invalid += 1;
            continue;
        }

        let id = parts[0];
        let qty = parts[1].parse::<i32>().unwrap_or(0);
        let status = if parts.len() >= 3 { parts[2] } else { "ok" };

        if qty >= 0 && (status == "ok" || status == "warn") {
            valid += 1;
            accepted.push(id);
        } else {
            invalid += 1;
            rejected.push(id);
        }
    }

    println!("valid: {}", valid);
    println!("invalid: {}", invalid);
    println!("accepted ids: {}", accepted.join(","));
    println!("rejected ids: {}", rejected.join(","));
}
