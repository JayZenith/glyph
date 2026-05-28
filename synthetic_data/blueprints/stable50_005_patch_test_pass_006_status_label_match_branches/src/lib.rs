#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Queued,
    Running { started_secs_ago: u64 },
    Finished { success: bool },
    Cancelled { by_user: bool },
}

pub fn state_label(state: JobState) -> &'static str {
    match state {
        JobState::Queued => "queued",
        JobState::Running { .. } => "active",
        JobState::Finished { success: true } => "ok",
        JobState::Finished { success: false } => "error",
        JobState::Cancelled { by_user: true } => "cancelled",
        JobState::Cancelled { by_user: false } => "cancelled",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queued_and_running_have_distinct_labels() {
        assert_eq!(state_label(JobState::Queued), "queued");
        assert_eq!(
            state_label(JobState::Running {
                started_secs_ago: 5
            }),
            "running"
        );
    }

    #[test]
    fn finished_variants_map_to_success_or_failure() {
        assert_eq!(
            state_label(JobState::Finished { success: true }),
            "success"
        );
        assert_eq!(
            state_label(JobState::Finished { success: false }),
            "failure"
        );
    }

    #[test]
    fn cancelled_source_is_reported() {
        assert_eq!(
            state_label(JobState::Cancelled { by_user: true }),
            "cancelled_by_user"
        );
        assert_eq!(
            state_label(JobState::Cancelled { by_user: false }),
            "cancelled_system"
        );
    }
}
