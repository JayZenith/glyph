const INPUT: &str = "id=ok-1|name=Ana|age=31|country=PT\nid=missing-country|name=Bo|age=22\nid=missing-age|name=Cy|country=US\nid=bad-age|name=Di|age=0|country=CA\nid=extra-field-order|country=DE|name=Eli|id=extra-field-order\nid=ok-2|country=JP|name=Fae|age=44";

#[derive(Default)]
struct Record {
    id: String,
    name: String,
    age: Option<u32>,
    country: String,
}

fn parse_record(line: &str) -> Record {
    let mut rec = Record::default();
    for part in line.split('|') {
        if let Some((k, v)) = part.split_once('=') {
            match k {
                "id" => rec.id = v.to_string(),
                "name" => rec.name = v.to_string(),
                "age" => rec.age = v.parse().ok(),
                "country" => rec.country = v.to_string(),
                _ => {}
            }
        }
    }
    rec
}

fn is_valid(rec: &Record) -> bool {
    !rec.id.is_empty() && !rec.name.is_empty() && rec.age.is_some()
}

fn main() {
    let mut valid = 0usize;
    let mut invalid = Vec::new();

    for line in INPUT.lines() {
        let rec = parse_record(line);
        if is_valid(&rec) {
            valid += 1;
        } else {
            invalid.push(rec.id);
        }
    }

    println!("valid={}", valid);
    println!("invalid={}", invalid.len());
    println!("invalid_ids={}", invalid.join(","));
}
