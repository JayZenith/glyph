fn main() {
    let values = [1, 2, 3, 4, 5, 6, 7, 8];

    let words = values
        .iter()
        .filter(|n| **n % 2 == 0)
        .filter_map(|n| match *n * *n {
            4 => Some("two"),
            16 => Some("four"),
            36 => Some("six"),
            64 => Some("eight"),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(",");

    print!("{}", words);
}
