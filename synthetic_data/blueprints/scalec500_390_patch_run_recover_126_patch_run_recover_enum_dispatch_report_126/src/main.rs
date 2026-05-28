enum Command {
    Create { name: &'static str },
    Move { name: &'static str, dest: &'static str },
    Tag { name: &'static str, tag: &'static str },
    Delete { name: &'static str },
}

fn describe(cmd: &Command) -> String {
    match cmd {
        Command::Create { name } => format!("create {}", name),
        Command::Move { name, dest } => format!("moved {} to {}", name, dest),
        Command::Tag { name, tag } => format!("tagged {} ({})", tag, name),
        Command::Delete { name } => format!("removed {}", name),
    }
}

fn main() {
    let commands = [
        Command::Create { name: "alpha" },
        Command::Move {
            name: "beta",
            dest: "trash",
        },
        Command::Tag {
            name: "gamma",
            tag: "hot",
        },
        Command::Delete { name: "delta" },
    ];

    for cmd in commands.iter() {
        println!("{}", describe(cmd));
    }
}
