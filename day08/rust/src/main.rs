#![feature(let_chains)]

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use std::time::Instant;

fn main() {
    let input = include_str!("../../../inputs/input_08.txt");

    time(|| {
        // <1ms
        println!("First part: {}", solve(input));
    });

    time(|| {
        // <1ms
        println!("Bonus: {}", bonus(input));
    });
}

fn time<F>(mut f: F)
where
    F: FnMut(),
{
    let t0 = Instant::now();
    f();
    println!("  took {:?}", t0.elapsed());
}

fn solve(input: &str) -> usize {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut antennas = FxHashMap::default();

    for y in 0..h {
        for x in 0..w {
            let a = grid[y as usize][x as usize];
            if a != '.' {
                antennas
                    .entry(a)
                    .and_modify(|locations: &mut Vec<(i32, i32)>| {
                        locations.push((x, y));
                    })
                    .or_insert(vec![(x, y)]);
            }
        }
    }

    let mut antinodes = FxHashSet::default();

    for (_a, locations) in antennas {
        for pair in locations.into_iter().combinations(2) {
            let (ax, ay) = pair[0];
            let (bx, by) = pair[1];
            let ri = (bx + (bx - ax), by + (by - ay));
            let le = (ax + (ax - bx), ay + (ay - by));
            antinodes.insert(ri);
            antinodes.insert(le);
        }
    }

    antinodes
        .into_iter()
        .filter(|&(x, y)| x >= 0 && x < w && y >= 0 && y < h)
        .count()
}

fn bonus(input: &str) -> usize {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut antennas = FxHashMap::default();

    for y in 0..h {
        for x in 0..w {
            let a = grid[y as usize][x as usize];
            if a != '.' {
                antennas
                    .entry(a)
                    .and_modify(|locations: &mut Vec<(i32, i32)>| {
                        locations.push((x, y));
                    })
                    .or_insert(vec![(x, y)]);
            }
        }
    }

    let mut antinodes = FxHashSet::default();

    for (_a, locations) in antennas {
        for pair in locations.into_iter().combinations(2) {
            let (ax, ay) = pair[0];
            let (bx, by) = pair[1];
            let (dx, dy) = (bx - ax, by - ay);

            let (mut rx, mut ry) = (bx, by);
            while rx >= 0 && rx < w && ry >= 0 && ry < h {
                antinodes.insert((rx, ry));
                rx += dx;
                ry += dy;
            }

            let (mut lx, mut ly) = (ax, ay);
            while lx >= 0 && lx < w && ly >= 0 && ly < h {
                antinodes.insert((lx, ly));
                lx -= dx;
                ly -= dy;
            }
        }
    }

    antinodes.into_iter().count()
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
        ),
        14
    );

    assert_eq!(
        bonus(
            "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
        ),
        34
    );
}
