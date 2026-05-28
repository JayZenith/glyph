#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Ping,
    Fetch { cached: bool, retries: u8 },
    Save { dry_run: bool },
    Remove { force: bool, exists: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    ReplyPong,
    UseCache,
    NetworkFetch,
    RetryFetch,
    ValidateOnly,
    Persist,
    SkipMissing,
    Delete,
    Refuse,
}

pub fn decide(input: Input) -> Action {
    match input {
        Input::Ping => Action::ReplyPong,
        Input::Fetch {
            cached: true,
            retries: _,
        } => Action::UseCache,
        Input::Fetch {
            cached: false,
            retries: 0,
        } => Action::NetworkFetch,
        Input::Fetch {
            cached: false,
            retries: _,
        } => Action::RetryFetch,
        Input::Save { dry_run: true } => Action::ValidateOnly,
        Input::Save { dry_run: false } => Action::Persist,
        Input::Remove {
            force: _,
            exists: false,
        } => Action::SkipMissing,
        Input::Remove {
            force: true,
            exists: true,
        } => Action::Delete,
        Input::Remove {
            force: false,
            exists: true,
        } => Action::Refuse,
    }
}

#[cfg(test)]
mod tests {
    use super::{decide, Action, Input};

    #[test]
    fn ping_returns_pong() {
        assert_eq!(decide(Input::Ping), Action::ReplyPong);
    }

    #[test]
    fn fetch_prefers_cache_even_with_retries() {
        assert_eq!(
            decide(Input::Fetch {
                cached: true,
                retries: 9,
            }),
            Action::UseCache
        );
    }

    #[test]
    fn fetch_without_cache_and_no_retries_is_network_fetch() {
        assert_eq!(
            decide(Input::Fetch {
                cached: false,
                retries: 0,
            }),
            Action::NetworkFetch
        );
    }

    #[test]
    fn fetch_without_cache_and_some_retries_is_retry_fetch() {
        assert_eq!(
            decide(Input::Fetch {
                cached: false,
                retries: 2,
            }),
            Action::RetryFetch
        );
    }

    #[test]
    fn save_dry_run_only_validates() {
        assert_eq!(decide(Input::Save { dry_run: true }), Action::ValidateOnly);
    }

    #[test]
    fn save_real_run_persists() {
        assert_eq!(decide(Input::Save { dry_run: false }), Action::Persist);
    }

    #[test]
    fn missing_remove_is_skipped_even_if_forced() {
        assert_eq!(
            decide(Input::Remove {
                force: true,
                exists: false,
            }),
            Action::SkipMissing
        );
    }

    #[test]
    fn existing_remove_requires_force() {
        assert_eq!(
            decide(Input::Remove {
                force: false,
                exists: true,
            }),
            Action::Refuse
        );
        assert_eq!(
            decide(Input::Remove {
                force: true,
                exists: true,
            }),
            Action::Delete
        );
    }
}
