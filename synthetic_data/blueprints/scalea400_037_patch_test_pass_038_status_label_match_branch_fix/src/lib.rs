#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Queued,
    Running { retrying: bool },
    Finished { warnings: u32 },
    Failed { temporary: bool },
}

pub fn state_label(state: JobState) -> &'static str {
    match state {
        JobState::Queued => "queued",
        JobState::Running { retrying: true } => "running",
        JobState::Running { retrying: false } => "retrying",
        JobState::Finished { warnings } if warnings > 0 => "finished",
        JobState::Finished { .. } => "finished_clean",
        JobState::Failed { temporary: true } => "failed",
        JobState::Failed { temporary: false } => "failed_temp",
    }
}

#[cfg(test)]
mod tests {
    use super::{state_label, JobState};

    #[test]
    fn queued_and_running_labels() {
        assert_eq!(state_label(JobState::Queued), "queued");
        assert_eq!(state_label(JobState::Running { retrying: false }), "running");
        assert_eq!(state_label(JobState::Running { retrying: true }), "retrying");
    }

    #[test]
    fn finished_labels_depend_on_warnings() {
        assert_eq!(state_label(JobState::Finished { warnings: 0 }), "finished");
        assert_eq!(state_label(JobState::Finished { warnings: 2 }), "finished_with_warnings");
    }

    #[test]
    fn failed_labels_depend_on_temporary_flag() {
        assert_eq!(state_label(JobState::Failed { temporary: false }), "failed");
        assert_eq!(state_label(JobState::Failed { temporary: true }), "failed_temp");
    }
}
