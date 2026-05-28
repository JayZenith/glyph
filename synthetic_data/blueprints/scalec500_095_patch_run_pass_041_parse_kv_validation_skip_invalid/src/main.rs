fn parse_line(line: &str) -> Option<(&str, i32)> {
    let (key, value) = line.split_once('=')?;
    let n = value.parse::<i32>().ok()?;
    Some((key, n))
}

fn main() {
    let input = ["apples=12", "bananas=-3", "=7", "pears=5", "grape=x", "melon=4=2"];
    let mut ok = 0;
    let mut sum = 0;
    let mut bad = 0;

    for line in input {
        match parse_line(line) {
            Some((_k, v)) => {
                ok += 1;
                sum += v;
            }
            None => bad += 1,
        }
    }

    println!("ok:{} sum:{} bad:{}", ok, sum, bad);
}
