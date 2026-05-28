#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Fast,
    Safe,
    Audit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Allow,
    Deny,
    Log(String),
}

pub fn decide(mode: Mode, flagged: bool, note: Option<&str>) -> Action {
    match mode {
        Mode::Fast => {
            if flagged {
                Action::Deny
            } else {
                Action::Allow
            }
        }
        Mode::Safe => {
            if flagged {
                Action::Log(note.unwrap_or("flagged").to_string())
            } else {
                Action::Allow
            }
        }
        Mode::Audit => {
            if flagged {
                Action::Deny
            } else {
                Action::Log(note.unwrap_or("audit").to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fast_denies_only_when_flagged() {
        assert_eq!(decide(Mode::Fast, false, None), Action::Allow);
        assert_eq!(decide(Mode::Fast, true, Some("x")), Action::Deny);
    }

    #[test]
    fn safe_mode_denies_when_flagged() {
        assert_eq!(decide(Mode::Safe, true, Some("fraud")), Action::Deny);
    }

    #[test]
    fn safe_mode_logs_review_when_clean() {
        assert_eq!(decide(Mode::Safe, false, None), Action::Log("review".to_string()));
    }

    #[test]
    fn audit_always_logs_and_prefers_note() {
        assert_eq!(decide(Mode::Audit, true, Some("trace")), Action::Log("trace".to_string()));
        assert_eq!(decide(Mode::Audit, false, None), Action::Log("audit".to_string()));
    }
}
