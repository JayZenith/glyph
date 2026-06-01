const INPUT: &str = "1|ok|alpha\n\n2|warn|beta\nxx|OK|bad-id\n3|oops|gamma\n4|ERR\n5|err|delta\n";

fn main() {
    let mut valid = Vec::new();
    let mut invalid = 0usize;

    for line in INPUT.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 3 {
            invalid += 1;
            continue;
        }

        let id = parts[0];
        let status = parts[1];
        let name = parts[2];

        if !id.chars().all(|c| c.is_ascii_digit()) {
            invalid += 1;
            continue;
        }

        if status != "OK" && status != "WARN" && status != "ERR" {
            invalid += 1;
            continue;
        }

        valid.push(format!("{}:{}:{}", id, status, name));
    }

    println!("valid={} invalid={}", valid.len(), invalid);
    for row in valid {
        println!("{}", row);
    }
}
