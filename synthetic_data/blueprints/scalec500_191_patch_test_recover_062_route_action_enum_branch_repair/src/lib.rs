#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Ingest,
    Export,
    Audit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    DryRun,
    Execute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Queue,
    Send,
    Store,
    Log,
    Review,
    Reject,
}

pub fn decide_action(route: Route, mode: Mode, urgent: bool, authorized: bool) -> Action {
    match (route, mode, urgent, authorized) {
        (Route::Ingest, Mode::DryRun, _, _) => Action::Log,
        (Route::Ingest, Mode::Execute, true, _) => Action::Send,
        (Route::Ingest, Mode::Execute, false, _) => Action::Store,

        (Route::Export, Mode::DryRun, _, _) => Action::Queue,
        (Route::Export, Mode::Execute, _, true) => Action::Send,
        (Route::Export, Mode::Execute, _, false) => Action::Queue,

        (Route::Audit, Mode::DryRun, _, _) => Action::Review,
        (Route::Audit, Mode::Execute, true, true) => Action::Send,
        (Route::Audit, Mode::Execute, true, false) => Action::Log,
        (Route::Audit, Mode::Execute, false, true) => Action::Store,
        (Route::Audit, Mode::Execute, false, false) => Action::Reject,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingest_execute_respects_urgency() {
        assert_eq!(decide_action(Route::Ingest, Mode::Execute, true, false), Action::Queue);
        assert_eq!(decide_action(Route::Ingest, Mode::Execute, false, true), Action::Store);
    }

    #[test]
    fn export_dry_run_logs_instead_of_queueing() {
        assert_eq!(decide_action(Route::Export, Mode::DryRun, false, false), Action::Log);
    }

    #[test]
    fn export_execute_requires_authorization() {
        assert_eq!(decide_action(Route::Export, Mode::Execute, false, true), Action::Send);
        assert_eq!(decide_action(Route::Export, Mode::Execute, true, false), Action::Reject);
    }

    #[test]
    fn audit_execute_has_distinct_outcomes() {
        assert_eq!(decide_action(Route::Audit, Mode::Execute, true, true), Action::Send);
        assert_eq!(decide_action(Route::Audit, Mode::Execute, true, false), Action::Review);
        assert_eq!(decide_action(Route::Audit, Mode::Execute, false, true), Action::Store);
        assert_eq!(decide_action(Route::Audit, Mode::Execute, false, false), Action::Reject);
    }

    #[test]
    fn dry_run_never_rejects() {
        assert_eq!(decide_action(Route::Ingest, Mode::DryRun, true, false), Action::Log);
        assert_eq!(decide_action(Route::Export, Mode::DryRun, true, true), Action::Log);
        assert_eq!(decide_action(Route::Audit, Mode::DryRun, false, false), Action::Review);
    }
}
