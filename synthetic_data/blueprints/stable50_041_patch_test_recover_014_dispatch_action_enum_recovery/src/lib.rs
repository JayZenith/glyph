#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Ping,
    Fetch { key: String, refresh: bool },
    Store { key: String, value: String, overwrite: bool },
    Remove(Option<String>),
}

pub fn dispatch(req: Request) -> &'static str {
    match req {
        Request::Ping => "noop",
        Request::Fetch { refresh, .. } => {
            if refresh { "fetch-cache" } else { "fetch-live" }
        }
        Request::Store { overwrite, value, .. } => {
            if overwrite || value.is_empty() { "store-skip" } else { "store-new" }
        }
        Request::Remove(key) => match key {
            Some(_) => "remove-all",
            None => "remove-one",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{dispatch, Request};

    #[test]
    fn ping_is_ack() {
        assert_eq!(dispatch(Request::Ping), "ack");
    }

    #[test]
    fn fetch_without_refresh_uses_cache() {
        assert_eq!(
            dispatch(Request::Fetch {
                key: "users".into(),
                refresh: false,
            }),
            "fetch-cache"
        );
    }

    #[test]
    fn fetch_with_refresh_goes_live() {
        assert_eq!(
            dispatch(Request::Fetch {
                key: "users".into(),
                refresh: true,
            }),
            "fetch-live"
        );
    }

    #[test]
    fn store_empty_value_is_rejected_even_without_overwrite() {
        assert_eq!(
            dispatch(Request::Store {
                key: "token".into(),
                value: "".into(),
                overwrite: false,
            }),
            "store-reject"
        );
    }

    #[test]
    fn store_overwrite_replaces_existing() {
        assert_eq!(
            dispatch(Request::Store {
                key: "token".into(),
                value: "abc".into(),
                overwrite: true,
            }),
            "store-replace"
        );
    }

    #[test]
    fn store_new_value_without_overwrite_creates() {
        assert_eq!(
            dispatch(Request::Store {
                key: "token".into(),
                value: "abc".into(),
                overwrite: false,
            }),
            "store-new"
        );
    }

    #[test]
    fn remove_some_targets_one() {
        assert_eq!(dispatch(Request::Remove(Some("token".into()))), "remove-one");
    }

    #[test]
    fn remove_none_clears_all() {
        assert_eq!(dispatch(Request::Remove(None)), "remove-all");
    }
}
