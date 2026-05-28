#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Start,
    Stop,
    Pause,
    Resume,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Manual,
    Auto,
}

pub fn dispatch(action: Action, mode: Mode, urgent: bool) -> &'static str {
    match action {
        Action::Start => match mode {
            Mode::Manual => "spin-up",
            Mode::Auto => "boot",
        },
        Action::Stop => {
            if urgent {
                "halt"
            } else {
                "stop"
            }
        }
        Action::Pause => {
            if urgent {
                "halt"
            } else {
                "wait"
            }
        }
        Action::Resume => match mode {
            Mode::Manual => "spin-up",
            Mode::Auto => "resume",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{dispatch, Action, Mode};

    #[test]
    fn start_depends_on_mode() {
        assert_eq!(dispatch(Action::Start, Mode::Manual, false), "spin-up");
        assert_eq!(dispatch(Action::Start, Mode::Auto, false), "boot");
    }

    #[test]
    fn stop_urgent_halts_but_nonurgent_stops() {
        assert_eq!(dispatch(Action::Stop, Mode::Manual, true), "halt");
        assert_eq!(dispatch(Action::Stop, Mode::Auto, false), "stop");
    }

    #[test]
    fn pause_never_reuses_stop_labels() {
        assert_eq!(dispatch(Action::Pause, Mode::Manual, false), "pause");
        assert_eq!(dispatch(Action::Pause, Mode::Auto, true), "pause-now");
    }

    #[test]
    fn resume_is_distinct_from_start() {
        assert_eq!(dispatch(Action::Resume, Mode::Manual, false), "resume");
        assert_eq!(dispatch(Action::Resume, Mode::Auto, true), "resume");
    }
}
