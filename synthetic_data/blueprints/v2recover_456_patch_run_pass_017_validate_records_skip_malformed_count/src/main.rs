fn count_valid(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() != 3 {
                return false;
            }
            let status = parts[1];
            let score = parts[2].parse::<u32>().unwrap_or(0);
            status == "active" || score >= 10
        })
        .count()
}

fn main() {
    let data = "alice|active|12\nbob|inactive|20\ncharlie|active|9\ninvalid\ndana|active|10";
    println!("{}", count_valid(data));
}
