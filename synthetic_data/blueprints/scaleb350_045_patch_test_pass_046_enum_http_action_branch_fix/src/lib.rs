#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
}

pub fn classify(method: Method, has_body: bool, is_authenticated: bool) -> &'static str {
    match method {
        Method::Get | Method::Head => {
            if has_body {
                "invalid"
            } else if is_authenticated {
                "read"
            } else {
                "public"
            }
        }
        Method::Post | Method::Put => {
            if is_authenticated {
                "write"
            } else {
                "public"
            }
        }
        Method::Delete => {
            if has_body {
                "invalid"
            } else if is_authenticated {
                "write"
            } else {
                "public"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_and_head_without_body_are_read_only() {
        assert_eq!(classify(Method::Get, false, true), "read");
        assert_eq!(classify(Method::Head, false, true), "read");
        assert_eq!(classify(Method::Get, false, false), "public");
    }

    #[test]
    fn body_on_safe_methods_is_invalid() {
        assert_eq!(classify(Method::Get, true, true), "invalid");
        assert_eq!(classify(Method::Head, true, false), "invalid");
    }

    #[test]
    fn post_and_put_require_authentication_even_without_body() {
        assert_eq!(classify(Method::Post, true, true), "write");
        assert_eq!(classify(Method::Put, false, true), "write");
        assert_eq!(classify(Method::Post, true, false), "forbidden");
        assert_eq!(classify(Method::Put, false, false), "forbidden");
    }

    #[test]
    fn delete_requires_authentication_and_no_body() {
        assert_eq!(classify(Method::Delete, false, true), "write");
        assert_eq!(classify(Method::Delete, true, true), "invalid");
        assert_eq!(classify(Method::Delete, false, false), "forbidden");
    }
}
