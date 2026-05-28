const INPUT: &str = "alice,42,admin\nbob,0,user\ncarol,17,moderator\ndave,abc,user\nerin,25,guest";

fn valid_role(role: &str) -> bool {
    matches!(role, "admin" | "user" | "moderator")
}

fn count_valid(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut parts = line.split(',');
            let name = parts.next().unwrap_or("");
            let age = parts.next().unwrap_or("");
            let role = parts.next().unwrap_or("");

            !name.is_empty() && age.parse::<u32>().is_ok() && valid_role(role)
        })
        .count()
}

fn main() {
    let valid = count_valid(INPUT);
    let invalid = INPUT.lines().count() - valid;
    println!("valid={} invalid={}", valid, invalid);
}
