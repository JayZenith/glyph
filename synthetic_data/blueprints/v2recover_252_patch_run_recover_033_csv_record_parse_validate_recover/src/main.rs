const INPUT: &str = "101,apple,3\n102,banana,x\n,carrot,4\n104,date,5,extra\n105,,2\n106,pear,7";

fn main() {
    let mut ok = 0;
    let mut err = 0;
    let mut out = Vec::new();

    for line in INPUT.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 3 {
            err += 1;
            out.push(format!("ERR malformed"));
            continue;
        }

        let id = parts[0].trim();
        let name = parts[1].trim();
        let qty = parts[2].trim();

        if id.is_empty() {
            err += 1;
            out.push("ERR missing id".to_string());
        } else if qty.parse::<u32>().is_err() {
            err += 1;
            out.push(format!("ERR {} bad qty", id));
        } else {
            ok += 1;
            out.push(format!("OK {} {} {}", id, name, qty));
        }
    }

    out.push(format!("SUMMARY ok={} err={}", ok, err));
    println!("{}", out.join("\n"));
}
