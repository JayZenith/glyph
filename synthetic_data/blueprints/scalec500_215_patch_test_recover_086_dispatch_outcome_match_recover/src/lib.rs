pub enum Job {
    Upload { bytes: u64, secure: bool },
    Cleanup { recursive: bool },
    Ping,
    Query(Option<&'static str>),
}

pub fn outcome(job: Job) -> &'static str {
    match job {
        Job::Upload { .. } => "sent",
        Job::Cleanup { recursive } => {
            if recursive {
                "clean"
            } else {
                "deep-clean"
            }
        }
        Job::Ping => "pong",
        Job::Query(None) => "query:all",
        Job::Query(Some(_)) => "query:all",
    }
}

#[cfg(test)]
mod tests {
    use super::{outcome, Job};

    #[test]
    fn upload_depends_on_security() {
        assert_eq!(outcome(Job::Upload { bytes: 12, secure: true }), "sent-secure");
        assert_eq!(outcome(Job::Upload { bytes: 12, secure: false }), "sent");
    }

    #[test]
    fn cleanup_depends_on_recursive_flag() {
        assert_eq!(outcome(Job::Cleanup { recursive: true }), "deep-clean");
        assert_eq!(outcome(Job::Cleanup { recursive: false }), "clean");
    }

    #[test]
    fn query_distinguishes_targeted_requests() {
        assert_eq!(outcome(Job::Query(None)), "query:all");
        assert_eq!(outcome(Job::Query(Some("users"))), "query:one");
    }

    #[test]
    fn ping_is_unchanged() {
        assert_eq!(outcome(Job::Ping), "pong");
    }
}
