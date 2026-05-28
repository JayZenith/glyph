#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Request {
    Ping,
    Fetch { fresh: bool, cached: bool },
    Store { authenticated: bool, readonly: bool },
    Admin { authenticated: bool, action: AdminAction },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminAction {
    Restart,
    ReadMetrics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Immediate(&'static str),
    Queue(&'static str),
    Reject(&'static str),
}

pub fn decide(req: Request) -> Decision {
    match req {
        Request::Ping => Decision::Immediate("pong"),
        Request::Fetch { fresh: true, .. } => Decision::Immediate("origin"),
        Request::Fetch { fresh: false, cached: true } => Decision::Immediate("cache"),
        Request::Fetch { .. } => Decision::Reject("miss"),
        Request::Store { readonly: true, .. } => Decision::Reject("readonly"),
        Request::Store { authenticated: true, .. } => Decision::Immediate("write"),
        Request::Store { .. } => Decision::Queue("auth"),
        Request::Admin { authenticated: false, .. } => Decision::Reject("forbidden"),
        Request::Admin { action: AdminAction::Restart, .. } => Decision::Immediate("restart"),
        Request::Admin { action: AdminAction::ReadMetrics, .. } => Decision::Queue("metrics"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_prefers_cache_when_available_even_if_fresh_requested() {
        assert_eq!(
            decide(Request::Fetch {
                fresh: true,
                cached: true,
            }),
            Decision::Immediate("cache")
        );
    }

    #[test]
    fn fetch_without_cache_uses_origin_only_when_fresh_requested() {
        assert_eq!(
            decide(Request::Fetch {
                fresh: true,
                cached: false,
            }),
            Decision::Immediate("origin")
        );
        assert_eq!(
            decide(Request::Fetch {
                fresh: false,
                cached: false,
            }),
            Decision::Reject("miss")
        );
    }

    #[test]
    fn readonly_store_requires_auth_before_reporting_readonly() {
        assert_eq!(
            decide(Request::Store {
                authenticated: false,
                readonly: true,
            }),
            Decision::Queue("auth")
        );
        assert_eq!(
            decide(Request::Store {
                authenticated: true,
                readonly: true,
            }),
            Decision::Reject("readonly")
        );
    }

    #[test]
    fn admin_metrics_are_immediate_for_authenticated_users() {
        assert_eq!(
            decide(Request::Admin {
                authenticated: true,
                action: AdminAction::ReadMetrics,
            }),
            Decision::Immediate("metrics")
        );
    }

    #[test]
    fn admin_restart_still_requires_authentication() {
        assert_eq!(
            decide(Request::Admin {
                authenticated: false,
                action: AdminAction::Restart,
            }),
            Decision::Reject("forbidden")
        );
    }
}
