fn build_labels(values: &[i32]) -> String {
    values
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .filter(|&sq| sq > 10)
        .map(|sq| format!("n{}", sq))
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    let data = [1, 2, 3, 4, 5, 6];
    println!("{}", build_labels(&data));
}
