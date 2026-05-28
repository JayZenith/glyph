const INPUT: &str = "ts=100;level=INFO;user=alice;action=login
level=ERROR;ts=101;user=bob smith;action=download
user=carol;action=logout;ts=102
 ts=103;level=DEBUG;user=dana;action=login
level=WARN;ts=104;user=eve;action=download;ip=1.2.3.4
ts=10x5;level=INFO;user=frank;action=login
ts=106;level=ERROR;user=gary;action=logout;action=login";

#[derive(Default)]
struct Record {
    ts: Option<String>,
    level: Option<String>,
    user: Option<String>,
    action: Option<String>,
}

fn parse_line(line: &str) -> Option<Record> {
    let mut rec = Record::default();

    for part in line.split(';') {
        let (key, value) = part.split_once('=')?;
        match key {
            "ts" => rec.ts = Some(value.to_string()),
            "level" => rec.level = Some(value.to_string()),
            "user" => rec.user = Some(value.to_string()),
            "action" => rec.action = Some(value.to_string()),
            _ => {}
        }
    }

    Some(rec)
}

fn valid_level(s: &str) -> bool {
    matches!(s, "INFO" | "WARN" | "ERROR" | "DEBUG")
}

fn valid_action(s: &str) -> bool {
    matches!(s, "login" | "logout" | "download")
}

fn is_valid(rec: &Record) -> bool {
    match (&rec.ts, &rec.level, &rec.user, &rec.action) {
        (Some(ts), Some(level), Some(user), Some(action)) => {
            !ts.is_empty()
                && valid_level(level)
                && !user.is_empty()
                && valid_action(action)
        }
        _ => false,
    }
}

fn main() {
    let mut valid = 0;
    let mut invalid = 0;
    let mut out = Vec::new();

    for line in INPUT.lines() {
        if let Some(rec) = parse_line(line) {
            if is_valid(&rec) {
                valid += 1;
                out.push(format!(
                    "{}|{}|{}|{}",
                    rec.ts.unwrap(),
                    rec.level.unwrap(),
                    rec.user.unwrap(),
                    rec.action.unwrap()
                ));
            } else {
                invalid += 1;
            }
        } else {
            invalid += 1;
        }
    }

    for line in out {
        println!("{}", line);
    }
    println!("valid={} invalid={}", valid, invalid);
}
