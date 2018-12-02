fn main() {
    println!(
        "Checksum is {}",
        include_str!("puzzle-2-1/input")
            .lines()
            .fold([0, 0], |[a, b], line| {
                let r = line
                    .chars()
                    .fold((0, 0), |m, c| match line
                        .chars()
                        .filter(|&k| k == c)
                        .count() {
                            2 => (1, m.1),
                            3 => (m.0, 1),
                            _ => m,
                    });
                [a + r.0, b + r.1]
            })
            .into_iter()
            .product::<i32>(),
    );
}
