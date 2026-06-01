const INPUT: &str = "name,age,active\nalpha,34,true\nbeta,0,true\ngamma,12,TRUE\ndelta,9,false\nepsilon,27,true,extra\nzeta,44,true\neta,,true\ntheta,18,true\n";

fn main() {
    let mut out = Vec::new();

    for line in INPUT.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 3 {
            continue;
        }

        let name = parts[0];
        let age = parts[1].parse::<u32>().unwrap_or(0);
        let active = parts[2] == "true";

        if active && age >= 18 {
            out.push(name);
        }
    }

    println!("{}", out.join("\n"));
}
