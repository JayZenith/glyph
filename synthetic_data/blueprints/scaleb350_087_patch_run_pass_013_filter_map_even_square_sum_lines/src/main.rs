fn main() {
    let values = [1, 2, 3, 4, 5, 6];

    let kept: Vec<i32> = values
        .into_iter()
        .filter(|n| n % 2 == 1)
        .collect();

    let squares: Vec<i32> = kept.iter().map(|n| n * n).collect();
    let total: i32 = squares.iter().sum();

    println!("kept: {}", kept.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","));
    println!("squares: {}", squares.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","));
    println!("total: {}", total);
}
