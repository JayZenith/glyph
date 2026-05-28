#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Ping,
    Fetch { key: String, cached: bool },
    Mutate { dry_run: bool },
    Admin(AdminAction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminAction {
    FlushCache,
    RotateKeys,
    Audit { verbose: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Local,
    Cache,
    Write,
    Control,
    Deny,
}

pub fn decide(req: &Request, readonly_mode: bool, allow_admin: bool) -> Route {
    match req {
        Request::Ping => Route::Local,
        Request::Fetch { cached, .. } => {
            if *cached {
                Route::Cache
            } else {
                Route::Write
            }
        }
        Request::Mutate { dry_run } => {
            if readonly_mode || *dry_run {
                Route::Local
            } else {
                Route::Write
            }
        }
        Request::Admin(action) => match action {
            AdminAction::FlushCache => {
                if allow_admin {
                    Route::Control
                } else {
                    Route::Deny
                }
            }
            AdminAction::RotateKeys => Route::Control,
            AdminAction::Audit { verbose } => {
                if *verbose {
                    Route::Control
                } else {
                    Route::Local
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_uncached_goes_local_not_write() {
        let req = Request::Fetch {
            key: "item-1".into(),
            cached: false,
        };
        assert_eq!(decide(&req, false, false), Route::Local);
    }

    #[test]
    fn mutate_dry_run_stays_local_even_when_not_readonly() {
        let req = Request::Mutate { dry_run: true };
        assert_eq!(decide(&req, false, false), Route::Local);
    }

    #[test]
    fn mutate_real_write_is_denied_in_readonly_mode() {
        let req = Request::Mutate { dry_run: false };
        assert_eq!(decide(&req, true, true), Route::Deny);
    }

    #[test]
    fn rotate_keys_requires_admin_permission() {
        let req = Request::Admin(AdminAction::RotateKeys);
        assert_eq!(decide(&req, false, false), Route::Deny);
        assert_eq!(decide(&req, false, true), Route::Control);
    }

    #[test]
    fn audit_verbose_requires_admin_but_quiet_is_local() {
        let verbose = Request::Admin(AdminAction::Audit { verbose: true });
        let quiet = Request::Admin(AdminAction::Audit { verbose: false });
        assert_eq!(decide(&quiet, false, false), Route::Local);
        assert_eq!(decide(&verbose, false, false), Route::Deny);
        assert_eq!(decide(&verbose, false, true), Route::Control);
    }

    #[test]
    fn flush_cache_obeys_admin_permission() {
        let req = Request::Admin(AdminAction::FlushCache);
        assert_eq!(decide(&req, false, false), Route::Deny);
        assert_eq!(decide(&req, false, true), Route::Control);
    }
}
