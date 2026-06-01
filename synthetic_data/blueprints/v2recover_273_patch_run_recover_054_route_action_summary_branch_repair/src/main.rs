enum Method {
    Get,
    Post,
    Patch,
    Delete,
    Head,
    Options,
}

enum Target {
    Collection(&'static str),
    Item(&'static str, u32),
    Health,
}

struct Route {
    method: Method,
    target: Target,
}

fn action(method: &Method, target: &Target) -> (&'static str, u32) {
    match method {
        Method::Get => ("READ", 1),
        Method::Post => ("CREATE", 2),
        Method::Patch => ("EDIT", 3),
        Method::Delete => ("DELETE", 4),
        Method::Head => ("READ", 1),
        Method::Options => ("READ", 1),
    }
}

fn method_name(method: &Method) -> &'static str {
    match method {
        Method::Get => "GET",
        Method::Post => "POST",
        Method::Patch => "PATCH",
        Method::Delete => "DELETE",
        Method::Head => "HEAD",
        Method::Options => "OPTIONS",
    }
}

fn target_name(target: &Target) -> String {
    match target {
        Target::Collection(name) => format!("/{}", name),
        Target::Item(name, _) => format!("/{}", name),
        Target::Health => "/health".to_string(),
    }
}

fn subject(target: &Target) -> String {
    match target {
        Target::Collection(name) => name.to_string(),
        Target::Item(name, _) => name.to_string(),
        Target::Health => "system".to_string(),
    }
}

fn routes() -> Vec<Route> {
    vec![
        Route { method: Method::Get, target: Target::Collection("users") },
        Route { method: Method::Post, target: Target::Collection("users") },
        Route { method: Method::Patch, target: Target::Item("users", 42) },
        Route { method: Method::Delete, target: Target::Item("users", 42) },
        Route { method: Method::Head, target: Target::Health },
        Route { method: Method::Options, target: Target::Collection("users") },
    ]
}

fn main() {
    let mut total = 0;
    for route in routes() {
        let (verb, weight) = action(&route.method, &route.target);
        total += weight;
        println!(
            "{} {} -> {} {}",
            method_name(&route.method),
            target_name(&route.target),
            verb,
            subject(&route.target)
        );
    }
    println!("Total weight: {}", total);
}
