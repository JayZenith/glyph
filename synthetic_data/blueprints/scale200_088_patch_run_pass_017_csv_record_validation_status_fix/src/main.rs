fn validate_line(line: &str) -> String {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 3 {
        return format!("invalid: {}", line);
    }

    let name = parts[0].trim();
    let age = parts[1].trim();
    let status = parts[2].trim();

    let age_ok = age.parse::<u32>().map(|n| n > 0).unwrap_or(false);
    let status_ok = status == "active" || status == "inactive";

    if !name.is_empty() && age_ok || status_ok {
        format!("ok: {}", name)
    } else {
        format!("invalid: {}", name)
    }
}

fn main() {
    let input = [
        "alpha,34,active",
        "beta,0,inactive",
        "gamma,27,paused",
        "delta,,active",
    ];

    let out = input
        .iter()
        .map(|line| validate_line(line))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", out);
}
