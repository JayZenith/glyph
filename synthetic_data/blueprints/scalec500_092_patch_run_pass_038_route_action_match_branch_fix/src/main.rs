enum Request {
    Get(&'static str),
    Post(&'static str),
    Delete(&'static str),
}

fn action(req: &Request) -> String {
    match req {
        Request::Get(path) if *path == "/" => "serve home".to_string(),
        Request::Get(_) => "404 not found".to_string(),
        Request::Post(path) if path.starts_with("/items") => "create item".to_string(),
        Request::Delete(_) => "404 not found".to_string(),
    }
}

fn main() {
    let requests = [
        Request::Get("/"),
        Request::Post("/items"),
        Request::Delete("/items/9"),
        Request::Get("/missing"),
    ];

    for req in &requests {
        let label = match req {
            Request::Get(path) => format!("GET {}", path),
            Request::Post(path) => format!("POST {}", path),
            Request::Delete(path) => format!("DELETE {}", path),
        };
        println!("{} => {}", label, action(req));
    }
}
