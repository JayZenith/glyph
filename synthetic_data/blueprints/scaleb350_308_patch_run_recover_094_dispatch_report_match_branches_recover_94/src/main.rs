enum Command {
    Create { name: &'static str },
    Rename { from: &'static str, to: &'static str },
    Delete(&'static str),
    Tag { name: &'static str, tags: Vec<&'static str> },
    Noop,
}

#[derive(Default)]
struct Stats {
    created: usize,
    renamed: usize,
    deleted: usize,
    tagged: usize,
    ignored: usize,
}

fn describe(cmd: &Command, stats: &mut Stats) -> String {
    match cmd {
        Command::Create { name } => {
            stats.created += 1;
            format!("created: {}", name)
        }
        Command::Rename { from, to } => {
            stats.deleted += 1;
            format!("renamed: {}", from)
        }
        Command::Delete(name) => {
            stats.deleted += 1;
            format!("deleted: {}", name)
        }
        Command::Tag { name, tags } => {
            stats.ignored += 1;
            format!("tagged: {} [{}]", name, tags.join(","))
        }
        Command::Noop => {
            stats.ignored += 1;
            "ignored: noop".to_string()
        }
    }
}

fn main() {
    let cmds = vec![
        Command::Create { name: "alpha" },
        Command::Rename {
            from: "beta",
            to: "beta-2",
        },
        Command::Delete("gamma"),
        Command::Tag {
            name: "delta",
            tags: vec!["hot", "new"],
        },
        Command::Noop,
    ];

    let mut stats = Stats::default();
    let mut lines = Vec::new();
    for cmd in &cmds {
        lines.push(describe(cmd, &mut stats));
    }

    lines.push(format!(
        "summary c={} r={} d={} t={} i={}",
        stats.created, stats.renamed, stats.deleted, stats.tagged, stats.ignored
    ));

    print!("{}", lines.join("\n"));
}
