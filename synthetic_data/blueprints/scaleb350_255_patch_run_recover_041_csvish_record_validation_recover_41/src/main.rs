const INPUT: &str = "100,alice,true,3\n101,bob,false,x\n102,charlie\n10a,dana,true,4\n104,erin,maybe,5\n105,_eve,true,6\n107,zoe,false,7\n";

fn valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn main() {
    let mut out = Vec::new();

    for (i, line) in INPUT.lines().enumerate() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 3 {
            out.push(format!("ERR line {} missing fields", i + 1));
            continue;
        }

        let id = parts[0];
        let name = parts[1];
        let active = parts[2];
        let score = parts.get(3).copied().unwrap_or("0");

        if !id.chars().all(|c| c.is_ascii_digit()) {
            out.push(format!("ERR line {} invalid id: {}", i + 1, id));
            continue;
        }
        if !valid_name(name) {
            out.push(format!("ERR line {} invalid name: {}", i + 1, name));
            continue;
        }
        if active != "true" && active != "false" {
            out.push(format!("ERR line {} invalid active: {}", i + 1, active));
            continue;
        }
        if !score.chars().all(|c| c.is_ascii_digit()) {
            out.push(format!("ERR line {} invalid score: {}", i + 1, score));
            continue;
        }

        out.push(format!("OK {} {} {}", id, name, score));
    }

    print!("{}", out.join("\n"));
}
