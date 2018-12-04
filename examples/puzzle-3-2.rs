#![feature(test, extern_crate_item_prelude, split_ascii_whitespace, drain_filter)]
extern crate test;

fn exec(s: &str) -> usize {
    let m = s.lines().fold(Box::new([[0u32; 1000]; 1000]), |mut m, s| {
        Some(s
            .split(|c: char| !c.is_numeric())
            .filter(|s| s.len() != 0)
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
        ).map(|p| (0..p[3])
            .for_each(|i| (0..p[4])
            .for_each(|j| m[p[1] + i][p[2] + j] += 1
        )));
        m
    });
    s.lines().find_map(|s| {
        let p = s
            .split(|c: char| !c.is_numeric())
            .filter(|s| s.len() != 0)
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        for i in 0..p[3] {
            for j in 0..p[4] {
                    if m[p[1] + i][p[2] + j] > 1 {
                        return None;
                    }
            }
        }
        Some(p[0])
    }).unwrap()
}

fn exec_intersect(s: &str) -> usize {
    use packed_simd::i16x4;
    use std::collections::LinkedList;

    let mut rects = s
        .split(|c: char| !c.is_numeric())
        .filter(|s| s.len() != 0)
        .map(|e| e.parse::<i16>().unwrap())
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|p| (
                p[0],
                i16x4::new(p[1], p[1] + p[3] - 1, p[2], p[2] + p[4] - 1),
                i16x4::new(p[1] + p[3] - 1, p[1], p[2] + p[4] - 1, p[2]),
            )
        )
        .collect::<Vec<_>>();

    // The cost of sorting is slightly outweighed by the speed of elimination later on
    rects.sort_by(|a, b| a.1.extract(3).cmp(&b.1.extract(3)));

    return (rects
        .iter()
        .enumerate()
        .find(|(i, a)| rects
            .iter()
            .cycle()
            .skip(*i)
            .take(rects.len())
            .all(|b| ((a.1 - b.2) * i16x4::new(1, -1, 1, -1)).max(i16x4::splat(0)).wrapping_sum() != 0 || a.0 == b.0)
        ).unwrap().1).0 as usize;
}

fn main() {
    println!(
        "Non-overlapping square = {}",
        exec_intersect(include_str!("puzzle-3-2/input")),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[bench]
    fn bench(bench: &mut Bencher) {
        bench.iter(|| black_box(exec(include_str!("puzzle-3-2/input"))))
    }

    #[bench]
    fn bench_intersect(bench: &mut Bencher) {
        bench.iter(|| black_box(exec_intersect(include_str!("puzzle-3-2/input"))))
    }
}
