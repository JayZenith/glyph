fn main() {
    let values = [-3, -2, 0, 1, 2, 3, 4, 8];

    let out = values
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| format!("n={n}"))
        .collect::<Vec<_>>()
        .join(",");

    print!("{out}");
}
