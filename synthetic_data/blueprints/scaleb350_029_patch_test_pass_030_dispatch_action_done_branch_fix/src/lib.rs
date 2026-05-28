#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Init,
    Running { retries: u8 },
    Done { ok: bool },
}

pub fn dispatch(phase: Phase) -> &'static str {
    match phase {
        Phase::Init => "start",
        Phase::Running { retries } if retries >= 3 => "abort",
        Phase::Running { .. } => "retry",
        Phase::Done { ok: true } => "failed",
        Phase::Done { ok: false } => "failed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_starts() {
        assert_eq!(dispatch(Phase::Init), "start");
    }

    #[test]
    fn running_with_few_retries_retries() {
        assert_eq!(dispatch(Phase::Running { retries: 2 }), "retry");
    }

    #[test]
    fn running_with_many_retries_aborts() {
        assert_eq!(dispatch(Phase::Running { retries: 3 }), "abort");
        assert_eq!(dispatch(Phase::Running { retries: 9 }), "abort");
    }

    #[test]
    fn done_ok_reports_complete() {
        assert_eq!(dispatch(Phase::Done { ok: true }), "complete");
    }

    #[test]
    fn done_not_ok_reports_failed() {
        assert_eq!(dispatch(Phase::Done { ok: false }), "failed");
    }
}
