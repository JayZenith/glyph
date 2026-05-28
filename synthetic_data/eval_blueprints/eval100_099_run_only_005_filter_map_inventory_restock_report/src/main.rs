fn main() {
    let items = [
        ("bolts", Some(8), 12),
        ("nuts", Some(5), 7),
        ("washers", None, 4),
        ("screws", Some(14), 10),
        ("paint", Some(0), 3),
        ("glue", None, 6),
    ];

    let restocks: Vec<String> = items
        .iter()
        .filter_map(|(name, stock, min_needed)| {
            stock.and_then(|qty| {
                if qty < *min_needed {
                    Some(format!("{}=>{}", name, min_needed - qty))
                } else {
                    None
                }
            })
        })
        .collect();

    let backorder_total: i32 = items
        .iter()
        .filter_map(|(_, stock, min_needed)| stock.map(|qty| (*min_needed - qty).max(0)))
        .sum();

    println!("restock: {}", restocks.join(","));
    println!("backorder total: {}", backorder_total);
}
