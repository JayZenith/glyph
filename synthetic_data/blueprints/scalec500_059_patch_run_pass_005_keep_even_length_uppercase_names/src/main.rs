fn main() {
    let names = ["ava", "mila", "zoe", "noah", "liam", "ivy"];

    let picked = names
        .iter()
        .filter(|name| name.len() % 2 == 1)
        .map(|name| name.to_uppercase())
        .collect::<Vec<_>>()
        .join(", ");

    println!("{}", picked);
}
