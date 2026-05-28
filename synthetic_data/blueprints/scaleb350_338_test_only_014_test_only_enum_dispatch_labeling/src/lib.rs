#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Start,
    Stop,
    Pause { urgent: bool },
    Resume,
    Reset(u8),
}

pub fn describe(cmd: Command) -> &'static str {
    match cmd {
        Command::Start => "start",
        Command::Stop => "stop",
        Command::Pause { urgent: true } => "pause-now",
        Command::Pause { urgent: false } => "pause",
        Command::Resume => "resume",
        Command::Reset(0) => "noop-reset",
        Command::Reset(1..=3) => "soft-reset",
        Command::Reset(_) => "hard-reset",
    }
}

pub fn is_interrupting(cmd: Command) -> bool {
    match cmd {
        Command::Start | Command::Resume => false,
        Command::Pause { urgent } => urgent,
        Command::Stop => true,
        Command::Reset(level) => level >= 2,
    }
}

pub fn route(cmd: Command) -> (&'static str, bool) {
    match cmd {
        Command::Start => ("lifecycle", false),
        Command::Stop => ("lifecycle", true),
        Command::Pause { urgent } => ("scheduler", urgent),
        Command::Resume => ("scheduler", false),
        Command::Reset(level) if level <= 1 => ("maintenance", false),
        Command::Reset(_) => ("maintenance", true),
    }
}

#[cfg(test)]
mod tests {
    use super::{describe, is_interrupting, route, Command};

    #[test]
    fn describe_covers_every_branch() {
        assert_eq!(describe(Command::Start), "start");
        assert_eq!(describe(Command::Stop), "stop");
        assert_eq!(describe(Command::Pause { urgent: false }), "pause");
        assert_eq!(describe(Command::Pause { urgent: true }), "pause-now");
        assert_eq!(describe(Command::Resume), "resume");
        assert_eq!(describe(Command::Reset(0)), "noop-reset");
        assert_eq!(describe(Command::Reset(2)), "soft-reset");
        assert_eq!(describe(Command::Reset(7)), "hard-reset");
    }

    #[test]
    fn interrupting_depends_on_variant_details() {
        assert!(!is_interrupting(Command::Start));
        assert!(!is_interrupting(Command::Resume));
        assert!(!is_interrupting(Command::Pause { urgent: false }));
        assert!(is_interrupting(Command::Pause { urgent: true }));
        assert!(is_interrupting(Command::Stop));
        assert!(!is_interrupting(Command::Reset(1)));
        assert!(is_interrupting(Command::Reset(2)));
    }

    #[test]
    fn route_uses_category_and_escalation_rules() {
        assert_eq!(route(Command::Start), ("lifecycle", false));
        assert_eq!(route(Command::Stop), ("lifecycle", true));
        assert_eq!(route(Command::Pause { urgent: false }), ("scheduler", false));
        assert_eq!(route(Command::Pause { urgent: true }), ("scheduler", true));
        assert_eq!(route(Command::Resume), ("scheduler", false));
        assert_eq!(route(Command::Reset(0)), ("maintenance", false));
        assert_eq!(route(Command::Reset(1)), ("maintenance", false));
        assert_eq!(route(Command::Reset(2)), ("maintenance", true));
    }
}
