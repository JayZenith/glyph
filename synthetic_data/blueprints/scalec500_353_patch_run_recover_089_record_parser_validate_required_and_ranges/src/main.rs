fn main() {
    let input = "amy|23|true\nbob|17|true\nzoe|xx|false\n|max|true\neve|40|false\nivy|34|maybe\nneo|101|true\nkai|22";

    let mut users = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 2 {
            continue;
        }

        let user = parts[0];
        let age = parts[1].parse::<u8>().unwrap_or(0);

        if !user.is_empty() && age > 0 {
            users.push(user.to_string());
        }
    }

    users.sort();
    println!("valid users: {}", users.join(","));
}
