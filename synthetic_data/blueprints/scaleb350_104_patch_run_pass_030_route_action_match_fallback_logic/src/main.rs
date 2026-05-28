enum Route<'a> {
    Health,
    Status { ready: bool },
    Retry(u8),
    User(&'a str),
    Unknown,
}

fn parse_route(input: &str) -> Route<'_> {
    match input {
        "health" => Route::Health,
        "status" => Route::Status { ready: false },
        "retry" => Route::Retry(2),
        _ if input.starts_with("user/") => Route::User(&input[5..]),
        _ => Route::Unknown,
    }
}

fn action(route: Route<'_>) -> String {
    match route {
        Route::Health => "ok".to_string(),
        Route::Status { ready } => {
            if ready { "pending" } else { "ready" }.to_string()
        }
        Route::Retry(n) if n >= 3 => "retry-later".to_string(),
        Route::Retry(_) => "ok-now".to_string(),
        Route::User(id) if id.is_empty() => "unknown".to_string(),
        Route::User(id) => format!("user:{}", id),
        Route::Unknown => "unknown".to_string(),
    }
}

fn main() {
    let inputs = ["health", "status", "retry", "user/42", "other"];
    for input in inputs {
        let route = parse_route(input);
        println!("{} => {}", input, action(route));
    }
}
