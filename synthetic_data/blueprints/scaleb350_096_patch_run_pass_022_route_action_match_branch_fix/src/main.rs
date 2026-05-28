enum Method {
    Get,
    Post,
    Delete,
}

enum Route<'a> {
    Home,
    Asset(&'a str),
    Submit,
    User(&'a str),
}

struct Request<'a> {
    method: Method,
    route: Route<'a>,
}

fn action(req: &Request<'_>) -> String {
    match (&req.method, &req.route) {
        (Method::Get, Route::Home) => "serve home".to_string(),
        (Method::Get, Route::Asset(name)) => format!("serve static"),
        (Method::Post, Route::Submit) => "accept form".to_string(),
        (_, Route::User(_)) => "show user".to_string(),
        _ => "not found".to_string(),
    }
}

fn route_label(route: &Route<'_>) -> String {
    match route {
        Route::Home => "/".to_string(),
        Route::Asset(name) => format!("/assets/{name}"),
        Route::Submit => "/submit".to_string(),
        Route::User(id) => format!("/users/{id}"),
    }
}

fn method_label(method: &Method) -> &'static str {
    match method {
        Method::Get => "GET",
        Method::Post => "POST",
        Method::Delete => "DELETE",
    }
}

fn main() {
    let requests = [
        Request {
            method: Method::Get,
            route: Route::Home,
        },
        Request {
            method: Method::Get,
            route: Route::Asset("logo.png"),
        },
        Request {
            method: Method::Post,
            route: Route::Submit,
        },
        Request {
            method: Method::Delete,
            route: Route::User("42"),
        },
    ];

    for req in requests.iter() {
        println!(
            "{} {} -> {}",
            method_label(&req.method),
            route_label(&req.route),
            action(req)
        );
    }
}
