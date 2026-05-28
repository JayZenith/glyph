enum Action {
    Start,
    Stop,
    Pause,
    Flush,
    Refresh,
}

enum Route {
    Network,
    Storage,
    Ui,
}

fn route(action: Action) -> Route {
    match action {
        Action::Start | Action::Stop => Route::Network,
        Action::Pause => Route::Storage,
        Action::Flush => Route::Ui,
        Action::Refresh => Route::Storage,
    }
}

fn main() {
    let actions = [
        Action::Start,
        Action::Refresh,
        Action::Pause,
        Action::Flush,
        Action::Stop,
        Action::Refresh,
        Action::Start,
    ];

    let mut network = 0;
    let mut storage = 0;
    let mut ui = 0;

    for action in actions {
        match route(action) {
            Route::Network => network += 1,
            Route::Storage => storage += 1,
            Route::Ui => ui += 1,
        }
    }

    println!("network:{}", network);
    println!("storage:{}", storage);
    println!("ui:{}", ui);
}
