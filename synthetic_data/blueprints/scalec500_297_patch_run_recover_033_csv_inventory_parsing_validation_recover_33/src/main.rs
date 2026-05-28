fn main() {
    let input = "# inventory\napple,3\nbanana,5\n\npear,7\n,4\norange,x\ngrape,2,extra\nmelon\n";

    let mut valid = 0usize;
    let mut invalid = 0usize;
    let mut total_qty = 0i32;
    let mut kept: Vec<(String, i32)> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            invalid += 1;
            continue;
        }

        let name = parts[0].trim();
        let qty = parts[1].trim().parse::<i32>().unwrap_or(0);

        valid += 1;
        total_qty += qty;
        kept.push((name.to_string(), qty));
    }

    println!("valid={} invalid={} total_qty={}", valid, invalid, total_qty);
    for (name, qty) in kept {
        println!("{}:{}", name, qty);
    }
}
