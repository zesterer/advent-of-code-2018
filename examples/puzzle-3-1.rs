#![feature(split_ascii_whitespace)]

fn main() {
    println!(
        "Overlapping squares = {:?}",
        include_str!("puzzle-3-1/input")
            .lines()
            .fold(Box::new([[0u32; 1000]; 1000]), |mut map, s| {
                Some(s
                    .split(|c: char| !c.is_numeric())
                    .filter(|s| s.len() != 0)
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
                ).map(|p| (0..p[3])
                    .for_each(|i| (0..p[4])
                    .for_each(|j| map[p[1] + i][p[2] + j] += 1
                )));
                map
            })
            .iter()
            .map(|col| col.iter())
            .flatten()
            .map(|x| x.saturating_sub(1).min(1))
            .sum::<u32>(),
    );
}
