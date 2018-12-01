use std::collections::HashSet;

fn main() {
    println!(
        "First repeated frequency is {}",
        include_str!("puzzle-1-2/input")
            .lines()
            .cycle()
            .scan((0, HashSet::new()), |(a, p), x| {
                *a += x.parse::<i32>().unwrap();
                Some((*a, !p.insert(*a)))
            })
            .find(|i| i.1)
            .map(|i| i.0)
            .unwrap(),
    );
}
