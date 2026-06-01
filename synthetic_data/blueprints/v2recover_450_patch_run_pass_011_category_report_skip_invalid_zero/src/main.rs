use std::collections::BTreeMap;

fn main() {
    let input = [
        "Books,12",
        "Games,30",
        "Books,0",
        "BadRow",
        "Games,20",
        "Music,0",
        "Toys,xyz",
    ];

    let mut totals: BTreeMap<&str, (u32, i32)> = BTreeMap::new();
    let mut invalid = 0;

    for line in input {
        let Some((category, amount_text)) = line.split_once(',') else {
            invalid += 1;
            continue;
        };

        let amount: i32 = match amount_text.parse() {
            Ok(v) => v,
            Err(_) => {
                invalid += 1;
                continue;
            }
        };

        let entry = totals.entry(category).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += amount;
    }

    let mut lines = Vec::new();
    let mut grand_total = 0;
    let mut valid = 0;

    for (category, (count, total)) in totals {
        lines.push(format!("{}: count={} total={}", category, count, total));
        grand_total += total;
        valid += count;
    }

    lines.push(format!(
        "TOTAL valid={} invalid={} grand_total={}",
        valid, invalid, grand_total
    ));

    print!("{}", lines.join("\n"));
}
