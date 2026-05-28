#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Queued,
    Running { progress: u8 },
    Finished { ok: bool },
}

pub fn render_status(state: JobState, verbose: bool) -> String {
    match state {
        JobState::Queued => {
            if verbose {
                "queued (waiting)".to_string()
            } else {
                "queued".to_string()
            }
        }
        JobState::Running { progress } => {
            if verbose {
                format!("running: {}%", progress)
            } else {
                "running".to_string()
            }
        }
        JobState::Finished { ok } => {
            if verbose {
                if ok {
                    "done: ok".to_string()
                } else {
                    "done: failed".to_string()
                }
            } else {
                "done".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{render_status, JobState};

    #[test]
    fn queued_verbose_has_waiting_note() {
        assert_eq!(render_status(JobState::Queued, true), "queued (waiting)");
    }

    #[test]
    fn running_non_verbose_hides_progress() {
        assert_eq!(render_status(JobState::Running { progress: 42 }, false), "running");
    }

    #[test]
    fn running_verbose_includes_progress() {
        assert_eq!(render_status(JobState::Running { progress: 42 }, true), "running: 42%");
    }

    #[test]
    fn finished_success_non_verbose_is_ok() {
        assert_eq!(render_status(JobState::Finished { ok: true }, false), "ok");
    }

    #[test]
    fn finished_failure_non_verbose_is_failed() {
        assert_eq!(render_status(JobState::Finished { ok: false }, false), "failed");
    }

    #[test]
    fn finished_success_verbose_is_done_ok() {
        assert_eq!(render_status(JobState::Finished { ok: true }, true), "done: ok");
    }

    #[test]
    fn finished_failure_verbose_is_done_failed() {
        assert_eq!(render_status(JobState::Finished { ok: false }, true), "done: failed");
    }
}
