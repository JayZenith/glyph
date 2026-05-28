#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Queued,
    Running,
    Done { warnings: usize },
    Failed { retryable: bool },
    Cancelled,
}

pub fn state_label(state: JobState) -> &'static str {
    match state {
        JobState::Queued => "queued",
        JobState::Running => "running",
        JobState::Done { warnings } => {
            if warnings == 0 {
                "done"
            } else {
                "done"
            }
        }
        JobState::Failed { retryable } => {
            if retryable {
                "failed"
            } else {
                "cancelled"
            }
        }
        JobState::Cancelled => "failed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_cover_all_variants() {
        assert_eq!(state_label(JobState::Queued), "queued");
        assert_eq!(state_label(JobState::Running), "running");
        assert_eq!(state_label(JobState::Done { warnings: 0 }), "done");
        assert_eq!(state_label(JobState::Done { warnings: 2 }), "done_with_warnings");
        assert_eq!(state_label(JobState::Failed { retryable: true }), "retryable_failure");
        assert_eq!(state_label(JobState::Failed { retryable: false }), "failed");
        assert_eq!(state_label(JobState::Cancelled), "cancelled");
    }

    #[test]
    fn warning_count_does_not_affect_clean_done() {
        let clean = JobState::Done { warnings: 0 };
        let warned = JobState::Done { warnings: 1 };
        assert_ne!(state_label(clean), state_label(warned));
    }
}
