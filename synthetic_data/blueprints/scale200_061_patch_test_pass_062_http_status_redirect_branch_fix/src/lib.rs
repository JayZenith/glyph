#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatus {
    Ok,
    Created,
    MovedPermanently,
    TemporaryRedirect,
    NotFound,
    ServerError,
}

pub fn category(status: HttpStatus) -> &'static str {
    match status {
        HttpStatus::Ok | HttpStatus::Created => "success",
        HttpStatus::MovedPermanently | HttpStatus::TemporaryRedirect => "error",
        HttpStatus::NotFound => "client_error",
        HttpStatus::ServerError => "server_error",
    }
}

#[cfg(test)]
mod tests {
    use super::{category, HttpStatus};

    #[test]
    fn success_statuses_are_classified() {
        assert_eq!(category(HttpStatus::Ok), "success");
        assert_eq!(category(HttpStatus::Created), "success");
    }

    #[test]
    fn redirect_statuses_are_classified() {
        assert_eq!(category(HttpStatus::MovedPermanently), "redirect");
        assert_eq!(category(HttpStatus::TemporaryRedirect), "redirect");
    }

    #[test]
    fn error_statuses_are_classified() {
        assert_eq!(category(HttpStatus::NotFound), "client_error");
        assert_eq!(category(HttpStatus::ServerError), "server_error");
    }
}
