fn is_valid(line: &str) -> bool {
    let mut parts = line.split('|');
    let name = parts.next().unwrap_or("");
    let age = parts.next().unwrap_or("");
    let email = parts.next().unwrap_or("");

    if parts.next().is_some() {
        return false;
    }

    if name.is_empty() {
        return false;
    }

    let age_num: u32 = match age.parse() {
        Ok(n) => n,
        Err(_) => return false,
    };

    age_num >= 18 && email.contains('@')
}

fn main() {
    let input = "Alice|30|alice@example.com\nBob|17|bob@example.com\nCara|22|caratexample.com\nDan|19|dan@example.com|extra\nEve|18|eve@example.com";

    let valid = input.lines().filter(|line| is_valid(line)).count();
    println!("valid={}", valid);
}
