const INPUT: &str = "Alice,1001,88\nBob,22,91\n,2002,77\nCara,3003,100\nDylan,4004,101\nEve,5005,abc\nFrank,6006\n\n";

fn main() {
    let mut valid = Vec::new();
    let mut invalid = 0u32;
    let mut score_sum = 0u32;

    for line in INPUT.lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 3 {
            invalid += 1;
            continue;
        }

        let name = parts[0];
        let id = parts[1];
        let score = parts[2].parse::<u32>();

        if !name.is_empty() && id.len() >= 4 {
            if let Ok(score) = score {
                valid.push(format!("{}:{}:{}", name, id, score));
                score_sum += score;
            } else {
                invalid += 1;
            }
        } else {
            invalid += 1;
        }
    }

    for row in &valid {
        println!("valid: {}", row);
    }

    let avg = if valid.is_empty() {
        0.0
    } else {
        score_sum as f64 / valid.len() as f64
    };
    println!("invalid={}", invalid);
    println!("avg={:.1}", avg);
}
