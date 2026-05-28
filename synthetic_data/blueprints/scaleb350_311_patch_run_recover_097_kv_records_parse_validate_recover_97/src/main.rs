const INPUT: &str = r#"
# staff extract
id=1001;name=Ada;age=42;active=true
id=1002;name=Bob;age=17;active=true
id=1003;name=;age=20;active=true
id=10x4;name=Cia;age=33;active=true
id=1005;name=Dee;age=30;active=yes
id=1006;name=Fox;age=29;active=false
id=1007;name=Gia;age=30;active=false;note=vip
id=1008;name=Hal;age=30;active=true;age=31
# ignored

"#;

fn parse_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    let mut id = "";
    let mut name = "";
    let mut age = "";
    let mut active = "";

    for part in trimmed.split(';') {
        let (k, v) = part.split_once('=')?;
        match k.trim() {
            "id" => id = v.trim(),
            "name" => name = v.trim(),
            "age" => age = v.trim(),
            "active" => active = v.trim(),
            _ => {}
        }
    }

    if id.is_empty() || name.is_empty() || age.is_empty() || active.is_empty() {
        return None;
    }
    let age_num: u8 = age.parse().ok()?;
    if age_num < 18 {
        return None;
    }
    if active != "true" {
        return None;
    }
    Some(format!("{}:{}", id, name))
}

fn main() {
    let out: Vec<String> = INPUT.lines().filter_map(parse_line).collect();
    print!("{}", out.join("\n"));
}
