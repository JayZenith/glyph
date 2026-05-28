fn valid_names(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 3 {
            continue;
        }
        let name = parts[0].trim();
        let age: u32 = match parts[1].trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let active = parts[2].trim();
        if !name.is_empty() && age > 0 && active == "active" {
            out.push(name.to_string());
        }
    }
    out
}

fn main() {
    let data = "Ada|42|active
Bob|0|active
Cara|9|inactive
Dan|33
Eve|18|active
Fox|27|active|extra";

    for name in valid_names(data) {
        println!("{}", name);
    }
}
