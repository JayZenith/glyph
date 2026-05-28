const INPUT: &str = "alice:34\nBob:22\ncarol:-1\ndave\neve!:44\n1zed:30\nbob_2:18\nmallory:100\n:55\ntrent:17";

fn parse_valid_users(input: &str) -> Vec<&str> {
    let mut out = Vec::new();
    for line in input.lines() {
        if let Some((name, age_text)) = line.split_once(':') {
            if name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                if age_text.parse::<u8>().is_ok() {
                    out.push(name);
                }
            }
        }
    }
    out
}

fn main() {
    let users = parse_valid_users(INPUT);
    if users.is_empty() {
        println!("-");
    } else {
        println!("{}", users.join(","));
    }
}
