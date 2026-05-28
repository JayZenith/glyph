use std::collections::BTreeMap;

fn build_report(input: &str) -> String {
    let mut teams: BTreeMap<&str, BTreeMap<&str, u32>> = BTreeMap::new();

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let mut parts = line.split(',');
        let team = parts.next().unwrap();
        let month = parts.next().unwrap();
        let hours: u32 = parts.next().unwrap().parse().unwrap();

        let month_totals = teams.entry(team).or_default();
        month_totals.insert(month, hours);
    }

    let mut out = String::new();
    for (team, month_totals) in teams {
        out.push_str(&format!("Team {}\n", team));
        let mut team_total = 0;
        for (month, hours) in month_totals {
            team_total += hours;
            out.push_str(&format!("  {}: {}h\n", month, hours));
        }
        out.push_str(&format!("  total={}h\n", team_total));
    }

    out.trim_end().to_string()
}

fn main() {
    let data = "alpha,2024-01,5
alpha,2024-01,3
beta,2024-03,4
alpha,2024-02,2
beta,2024-01,4
beta,2024-03,1
";

    println!("{}", build_report(data));
}
