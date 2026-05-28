const INPUT: &str = "alpha,10\nbad-key,3\nbeta,xyz\ngamma,7,extra\nalpha,8\ngamma,9\n";

fn is_valid_key(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic())
}

fn main() {
    let mut valid = Vec::new();
    let mut invalid = Vec::new();

    for (idx, line) in INPUT.lines().enumerate() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            invalid.push(format!("{}: wrong field count", idx + 1));
            continue;
        }

        let key = parts[0];
        let value = parts[1];

        if !is_valid_key(key) {
            invalid.push(format!("{}: bad key", idx + 1));
            continue;
        }

        if value.parse::<u32>().is_err() {
            invalid.push(format!("{}: bad value", idx + 1));
            continue;
        }

        if !valid.iter().any(|k| k == key) {
            valid.push(key.to_string());
        }
    }

    println!("valid: {}", valid.join(","));
    println!("invalid:");
    for item in invalid {
        println!("{}", item);
    }
}
