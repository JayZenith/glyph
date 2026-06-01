enum Method {
    Get,
    Post,
    Delete,
    Patch,
}

enum Route {
    Users,
    User(u32),
    Status,
    Unknown(String),
}

struct Request {
    method: Method,
    route: Route,
}

fn parse(input: &str) -> Request {
    let mut parts = input.split_whitespace();
    let method = match parts.next().unwrap_or("") {
        "GET" => Method::Get,
        "POST" => Method::Post,
        "DELETE" => Method::Delete,
        _ => Method::Patch,
    };
    let path = parts.next().unwrap_or("");
    let route = match path {
        "/users" => Route::Users,
        "/status" => Route::Status,
        _ if path.starts_with("/users/") => {
            let tail = &path[7..];
            let id = tail.parse::<u32>().unwrap_or(0);
            Route::User(id)
        }
        _ => Route::Unknown(path.to_string()),
    };
    Request { method, route }
}

fn dispatch(req: &Request) -> String {
    match (&req.method, &req.route) {
        (Method::Get, Route::Users) => "GET /users -> list users".to_string(),
        (Method::Post, Route::Users) => "POST /users -> create user".to_string(),
        (Method::Get, Route::User(_)) => "GET /users/{id} -> show user".to_string(),
        (Method::Delete, Route::User(id)) => format!("DELETE /users/{id} -> removed"),
        (_, Route::Status) => "GET /status -> status ok".to_string(),
        (_, Route::Unknown(path)) => format!("GET {path} -> not found"),
        (Method::Patch, Route::User(id)) => format!("PATCH /users/{id} -> unsupported"),
        _ => "unhandled".to_string(),
    }
}

fn main() {
    let inputs = [
        "GET /users",
        "GET /users/42",
        "POST /users",
        "DELETE /users/42",
        "PATCH /users/42",
        "GET /status",
        "DELETE /status",
        "GET /unknown",
    ];

    for input in inputs {
        let req = parse(input);
        println!("{}", dispatch(&req));
    }
}
