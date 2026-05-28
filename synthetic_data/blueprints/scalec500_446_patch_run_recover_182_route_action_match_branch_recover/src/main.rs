enum Route {
    Api,
    Admin,
    Health,
    Unknown,
}

enum Method {
    Get,
    Post,
    Delete,
}

struct Request {
    path: &'static str,
    route: Route,
    method: Method,
}

fn decision(req: &Request) -> (&'static str, &'static str) {
    match req.route {
        Route::Api => match req.method {
            Method::Get => ("ALLOW", "read"),
            Method::Post => ("ALLOW", "write"),
            Method::Delete => ("DENY", "method-not-allowed"),
        },
        Route::Admin => match req.method {
            Method::Get => ("DENY", "admin-only"),
            Method::Post => ("ALLOW", "admin-write"),
            Method::Delete => ("DENY", "method-not-allowed"),
        },
        Route::Health => ("ALLOW", "health"),
        Route::Unknown => ("DENY", "not-found"),
    }
}

fn main() {
    let requests = [
        Request { path: "/api/items", route: Route::Api, method: Method::Get },
        Request { path: "/api/items", route: Route::Api, method: Method::Post },
        Request { path: "/admin", route: Route::Admin, method: Method::Get },
        Request { path: "/admin", route: Route::Admin, method: Method::Post },
        Request { path: "/health", route: Route::Health, method: Method::Get },
        Request { path: "/health", route: Route::Health, method: Method::Delete },
        Request { path: "/unknown", route: Route::Unknown, method: Method::Get },
    ];

    let mut allowed = 0;
    let mut denied = 0;

    for req in requests.iter() {
        let (status, reason) = decision(req);
        if status == "ALLOW" {
            allowed += 1;
        } else {
            denied += 1;
        }
        let method = match req.method {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Delete => "DELETE",
        };
        println!("{} {} {} -> {}", status, req.path, method, reason);
    }

    println!("summary: allowed={} denied={}", allowed, denied);
}
