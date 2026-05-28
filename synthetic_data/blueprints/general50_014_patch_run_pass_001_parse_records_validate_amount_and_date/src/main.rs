fn valid_record(line: &str) -> bool {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 3 {
        return false;
    }

    let id_ok = parts[0].starts_with("ID-") && parts[0][3..].chars().all(|c| c.is_ascii_digit());
    let date_ok = is_date(parts[1]);
    let amount_ok = parts[2].parse::<u32>().is_ok();

    id_ok && date_ok && amount_ok
}

fn is_date(s: &str) -> bool {
    let p: Vec<&str> = s.split('-').collect();
    if p.len() != 3 {
        return false;
    }
    if p[0].len() != 4 || p[1].len() != 2 || p[2].len() != 2 {
        return false;
    }
    p.iter().all(|part| part.chars().all(|c| c.is_ascii_digit()))
}

fn main() {
    let input = "ID-100|2024-01-02|50\nBAD|2024-01-02|7\nID-200|2024-13-01|9\nID-300|2024-05-10|x\nID-400|2024-12-31|0";
    let valid = input.lines().filter(|line| valid_record(line)).count();
    println!("valid={}", valid);
}
