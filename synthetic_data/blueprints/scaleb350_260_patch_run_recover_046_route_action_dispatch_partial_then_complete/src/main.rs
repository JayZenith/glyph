enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}

enum Route<'a> {
    Health,
    Users,
    UserId(u32),
    Admin,
    Unknown(&'a str),
}

struct Request<'a> {
    method: Method,
    route: Route<'a>,
    auth: bool,
}

fn dispatch(req: &Request) -> &'static str {
    match &req.route {
        Route::Health => match req.method {
            Method::Get => "ok",
            _ => "not-found",
        },
        Route::Users => match req.method {
            Method::Get => "list-users",
            Method::Post => "create-user",
            _ => "not-found",
        },
        Route::UserId(_) => match req.method {
            Method::Get => "show-user",
            Method::Patch => "show-user",
            Method::Delete => "delete-user",
            _ => "not-found",
        },
        Route::Admin => {
            if req.auth {
                "admin-panel"
            } else {
                "forbidden"
            }
        }
        Route::Unknown(_) => "method-not-allowed",
    }
}

fn route_label(route: &Route<'_>) -> String {
    match route {
        Route::Health => "/health".to_string(),
        Route::Users => "/users".to_string(),
        Route::UserId(id) => format!("/users/{id}"),
        Route::Admin => "/admin".to_string(),
        Route::Unknown(path) => path.to_string(),
    }
}

fn method_label(method: &Method) -> &'static str {
    match method {
        Method::Get => "GET",
        Method::Post => "POST",
        Method::Put => "PUT",
        Method::Patch => "PATCH",
        Method::Delete => "DELETE",
        Method::Head => "HEAD",
    }
}

fn main() {
    let requests = [
        Request {
            method: Method::Get,
            route: Route::Health,
            auth: false,
        },
        Request {
            method: Method::Post,
            route: Route::Users,
            auth: true,
        },
        Request {
            method: Method::Patch,
            route: Route::UserId(42),
            auth: true,
        },
        Request {
            method: Method::Delete,
            route: Route::UserId(42),
            auth: true,
        },
        Request {
            method: Method::Get,
            route: Route::Admin,
            auth: false,
        },
        Request {
            method: Method::Put,
            route: Route::Health,
            auth: false,
        },
        Request {
            method: Method::Head,
            route: Route::Unknown("/missing"),
            auth: false,
        },
    ];

    for req in requests {
        println!(
            "{} {} => {}",
            method_label(&req.method),
            route_label(&req.route),
            dispatch(&req)
        );
    }
}
