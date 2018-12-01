fn main() {
    println!(
        "Sum frequency is {}",
        include_str!("puzzle-1-1/input")
            .lines()
            .map(|s| s.parse::<i32>().unwrap())
            .sum::<i32>(),
    );
}
