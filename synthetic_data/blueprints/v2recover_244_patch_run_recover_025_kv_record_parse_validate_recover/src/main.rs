const INPUT: &str = "id=101;qty=3;active=yes\nid=102;qty=0;active=yes\nid=bad;qty=5;active=yes\nid=103;qty=8\nid=104;qty=2;active=no;note=skip\nid=105;qty=4;active=maybe\n";

fn valid_id(line: &str) -> Option<String> {
    let mut id = None;
    let mut qty = None;
    let mut active = None;

    for part in line.split(';') {
        let (key, value) = part.split_once('=')?;
        match key {
            "id" => id = Some(value.to_string()),
            "qty" => qty = value.parse::<u32>().ok(),
            "active" => active = Some(value == "yes"),
            _ => return None,
        }
    }

    if id.is_some() && qty.is_some() && active.is_some() {
        id
    } else {
        None
    }
}

fn main() {
    let ids: Vec<String> = INPUT.lines().filter_map(valid_id).collect();
    print!("{}", ids.join(","));
}
