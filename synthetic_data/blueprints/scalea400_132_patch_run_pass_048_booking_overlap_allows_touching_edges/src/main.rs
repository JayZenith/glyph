fn overlaps(a: (u32, u32), b: (u32, u32)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

fn can_book(existing: &[(u32, u32)], candidate: (u32, u32)) -> bool {
    !existing.iter().any(|&slot| overlaps(slot, candidate))
}

fn main() {
    let mut bookings = vec![(10, 20), (30, 40)];
    let requests = [(20, 25), (15, 18), (40, 45), (29, 31), (45, 50)];
    let mut accepted = 0;
    let mut rejected = 0;

    for req in requests {
        if can_book(&bookings, req) {
            bookings.push(req);
            accepted += 1;
        } else {
            rejected += 1;
        }
    }

    println!("accepted: {}", accepted);
    println!("rejected: {}", rejected);
}
