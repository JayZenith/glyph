fn main() {
    let values = [1, 2, 3, 4, 5];

    let lines: Vec<String> = values
        .iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .map(|n| format!("n={} sq={}", n, n * n))
        .collect();

    let sum: i32 = values
        .iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .sum();

    println!("{}\nsum={}", lines.join("\n"), sum);
}
