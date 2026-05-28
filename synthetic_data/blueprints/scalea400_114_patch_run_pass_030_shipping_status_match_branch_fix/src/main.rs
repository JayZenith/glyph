enum Route {
    Delivery { fragile: bool },
    Pickup,
    Returned,
    Missing,
}

struct Parcel {
    code: &'static str,
    route: Route,
}

fn describe(parcel: &Parcel) -> String {
    let action = match &parcel.route {
        Route::Delivery { fragile: true } => "hold at depot",
        Route::Delivery { fragile: false } => "deliver to door",
        Route::Pickup => "customer pickup",
        Route::Returned => "return to sender",
        Route::Missing => "lost package",
    };
    format!("{}: {}", parcel.code, action)
}

fn main() {
    let parcels = [
        Parcel {
            code: "A12",
            route: Route::Delivery { fragile: true },
        },
        Parcel {
            code: "B07",
            route: Route::Delivery { fragile: false },
        },
        Parcel {
            code: "C99",
            route: Route::Missing,
        },
        Parcel {
            code: "D10",
            route: Route::Returned,
        },
    ];

    for parcel in parcels.iter() {
        println!("{}", describe(parcel));
    }
}
