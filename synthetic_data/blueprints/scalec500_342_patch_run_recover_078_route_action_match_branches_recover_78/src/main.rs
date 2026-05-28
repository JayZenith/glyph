enum Action {
    List,
    Fetch { id: u32 },
    Create { id: Option<u32>, name: &'static str },
    Delete { id: Option<u32> },
    Patch { id: Option<u32>, field: &'static str, value: &'static str },
}

fn render(action: &Action) -> String {
    match action {
        Action::List => "GET /users -> list users".to_string(),
        Action::Fetch { id } => format!("GET /users/{id} -> fetch user {id}"),
        Action::Create { id: Some(id), name } => format!("POST /users/{id} -> create user {name}"),
        Action::Create { id: None, name } => format!("POST /users -> create user {name}"),
        Action::Delete { id: Some(id) } => format!("DELETE /users/{id} -> delete user {id}"),
        Action::Delete { id: None } => "DELETE /users -> delete all users".to_string(),
        Action::Patch { id: Some(id), field, value } => {
            format!("PATCH /users/{id} -> replace user {id} {field}={value}")
        }
        Action::Patch { id: None, field, value } => {
            format!("PATCH /users -> patch all users {field}={value}")
        }
    }
}

fn main() {
    let actions = [
        Action::List,
        Action::Fetch { id: 42 },
        Action::Create {
            id: None,
            name: "Alice",
        },
        Action::Create {
            id: Some(42),
            name: "Alice",
        },
        Action::Delete { id: Some(42) },
        Action::Delete { id: None },
        Action::Patch {
            id: Some(42),
            field: "role",
            value: "admin",
        },
        Action::Patch {
            id: None,
            field: "role",
            value: "admin",
        },
    ];

    for (i, action) in actions.iter().enumerate() {
        if i > 0 {
            println!();
        }
        print!("{}", render(action));
    }
}
