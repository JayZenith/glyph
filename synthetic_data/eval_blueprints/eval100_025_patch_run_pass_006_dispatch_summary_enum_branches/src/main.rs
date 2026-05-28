enum Command {
    Send(u8),
    Retry(u8),
    Archive,
    Noop,
}

fn describe(cmd: Command) -> String {
    match cmd {
        Command::Send(n) if n <= 1 => format!("send({n}) -> queued {n}"),
        Command::Send(n) => format!("send({n}) -> queued {n}"),
        Command::Retry(0) => "retry(0) -> retrying 0".to_string(),
        Command::Retry(n) => format!("retry({n}) -> retrying {n}"),
        Command::Archive => "archive -> archived".to_string(),
        Command::Noop => "noop -> no-op".to_string(),
    }
}

fn main() {
    let cmds = [
        Command::Archive,
        Command::Send(0),
        Command::Send(2),
        Command::Send(1),
        Command::Retry(3),
        Command::Retry(0),
        Command::Noop,
    ];

    for cmd in cmds {
        println!("{}", describe(cmd));
    }
}
