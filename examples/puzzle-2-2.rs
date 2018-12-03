#![feature(test, extern_crate_item_prelude)]
extern crate test;

fn exec(s: &str) -> Option<String> {
    s.lines().find_map(|x| s.lines().find_map(|y| match &x
        .chars()
        .zip(y.chars())
        .filter(|(a, b)| a == b)
        .map(|c| c.0)
        .collect::<String>()
    {
        s if s.len() + 1 == x.len() => Some(s.clone()),
        _ => None,
    }))
}

fn exec_ultra(l: &[u8]) -> usize {
    use std::ops::Sub;

    use packed_simd::{i8x4, u8x4, u8x8, u8x16, u8x32, u16x4, u16x8, u32x2};

    let mut rail: [u8x32; 250] = unsafe { std::mem::uninitialized() };
    for (i, c) in l.chunks(27).enumerate() {
        let mut ptr = unsafe { std::slice::from_raw_parts_mut(&mut rail[i] as *mut _ as *mut u8, 32) };
        ptr[0..26].copy_from_slice(&c[0..26]);
        ptr[26..32].copy_from_slice(&[0, 0, 0, 0, 0, 0])
    }

    let mut hashes: [[u16; 4]; 250] = unsafe { std::mem::uninitialized() };
    let rail_halves = unsafe { &*std::mem::transmute::<_, *mut [u8x16; 500]>(&mut rail as *mut _ as *mut [u8x32; 250]) };
    for i in 0..250 {
        unsafe {
            hashes[i][0] = rail_halves.get_unchecked(i * 2 + 0).wrapping_sum() as u16;
            hashes[i][2] = rail_halves.get_unchecked(i * 2 + 1).wrapping_sum() as u16;
            //hashes[i][2] = rail_halves.get_unchecked(i * 4 + 2).wrapping_sum() as u16;
            //hashes[i][3] = rail_halves.get_unchecked(i * 4 + 3).wrapping_sum() as u16;
        }
    }

    //let hash_rail = unsafe { &*std::mem::transmute::<_, *mut [u16x4; 250]>(&mut hashes as *mut _ as *mut [[u8; 8]; 250]) };

    for i in 0..250 {
        for j in i + 1..250 {
            if
                //(hash_rail[i] - hash_rail[j]).min(u16x4::splat(1)).wrapping_sum() == 1
                hashes[i][0].wrapping_sub(hashes[j][0]).min(1) +
                hashes[i][2].wrapping_sub(hashes[j][2]).min(1) == 1
            {
                if (rail[i] - rail[j]).min(u8x32::splat(1)).wrapping_sum() == 1 {
                    return i;
                }
            }
        }
    }

    return 0;
}

fn exec_fast(l: &[u8]) -> usize {
    // I challenge you, whoever the hell you are, to write an implementation faster than this!

    // Uh huh.

    // Didn't think so.

    use packed_simd::u8x32;

    let mut rail = [unsafe { std::mem::uninitialized::<u8x32>() }; 250];
    for (i, c) in l.chunks(27).enumerate() {
        let mut ptr = unsafe { std::slice::from_raw_parts_mut(&mut rail[i] as *mut _ as *mut u8, 32) };
        ptr[0..26].copy_from_slice(&c[0..26]);
        ptr[26..32].copy_from_slice(&[0, 0, 0, 0, 0, 0])
    }

    for i in 0..250 {
        for j in i + 1..250 {
            if (rail[i] - rail[j]).min(u8x32::splat(1)).wrapping_sum() == 1 {
                return i;
            }
        }
    }

    return 0;
}

pub fn exec_globi_old(input: &[&[u8]]) -> String {
    use itertools::Itertools;

    input.iter()
        .tuple_combinations()
        .find_map(|(box1, box2)| {
            let mut diff_indexes = box1.iter().zip(box2.iter())
                .enumerate()
                .filter(|(_, (c1, c2))| c1 != c2)
                .map(|(idx, _)| idx);

            match (diff_indexes.next(), diff_indexes.next()) {
                (Some(idx), None) => /* Only one diff */ {
                    let mut common = box1.to_vec();
                    common.remove(idx);
                    unsafe { Some(String::from_utf8_unchecked(common)) }
                }
                _ => None
            }
        }).expect("No solution")
}

pub fn exec_cryze<'a>(lines: impl Iterator<Item = &'a str>) -> Option<String> {
    use packed_simd::u8x32;

    #[repr(align(32))]
    #[derive(Copy, Clone)]
    struct Line([u8; 32]);

    let mut storage = [u8x32::splat(0); 250];
    let mut buf = Line([0; 32]);
    for (storage, line) in storage.iter_mut().zip(lines) {
        let line = line.trim_end();
        buf.0[..line.len()].copy_from_slice(line.as_bytes());
        *storage = u8x32::from_slice_aligned(&buf.0);
    }

    for (i, &a) in storage.iter().enumerate() {
        for &b in &storage[i + 1..] {
            if a.eq(b).select(u8x32::splat(1), u8x32::splat(0)).wrapping_sum() == 31 {
                let mut buf = String::with_capacity(25);
                let a: [u8; 32] = a.into();
                let b: [u8; 32] = b.into();
                for (&a, &b) in a.iter().zip(&b) {
                    if a == b {
                        buf.push(a as char);
                    }
                }
                return Some(buf);
            }
        }
    }

    None
}

fn main() {
    let lines = include_str!("puzzle-2-2/input").lines().collect::<Vec<_>>();
    println!(
        "Common letters are {}",
        lines[exec_ultra(include_bytes!("puzzle-2-2/input"))],
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[bench]
    fn bench(bench: &mut Bencher) {
        bench.iter(|| black_box(exec(include_str!("puzzle-2-2/input"))))
    }

    #[bench]
    fn bench_fast(bench: &mut Bencher) {
        let b = include_str!("puzzle-2-2/input").bytes().collect::<Vec<_>>();
        let l = b.split(|&b| b == b'\n').collect::<Vec<_>>();
        bench.iter(|| black_box(exec_fast(&b)))
    }

    #[bench]
    fn bench_ultra(bench: &mut Bencher) {
        let b = include_str!("puzzle-2-2/input").bytes().collect::<Vec<_>>();
        let l = b.split(|&b| b == b'\n').collect::<Vec<_>>();
        bench.iter(|| black_box(exec_ultra(&b)))
    }

    #[bench]
    fn bench_globi_old(bench: &mut Bencher) {
        let b = include_bytes!("puzzle-2-2/input").split(|&c| c == b'\n').collect::<Vec<_>>();
        bench.iter(|| black_box(exec_globi_old(&b)))
    }

    #[bench]
    fn bench_cryze(bench: &mut Bencher) {
        bench.iter(|| black_box(exec_cryze(include_str!("puzzle-2-2/input").lines())))
    }
}
