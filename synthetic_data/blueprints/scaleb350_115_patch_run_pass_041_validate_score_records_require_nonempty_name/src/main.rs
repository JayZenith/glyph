fn parse_record(line: &str) -> Option<(&str, u32)> {
    let (name, score_text) = line.split_once(':')?;
    let score: u32 = score_text.parse().ok()?;
    Some((name, score))
}

fn main() {
    let input = "ann:12\n:9\nbob:3\ncarol:x\ndave\nellen:4:7";

    let mut valid = 0;
    let mut invalid = 0;
    let mut total = 0;

    for line in input.lines() {
        match parse_record(line) {
            Some((_name, score)) => {
                valid += 1;
                total += score;
            }
            None => invalid += 1,
        }
    }

    println!("valid={} invalid={} total={}", valid, invalid, total);
}
