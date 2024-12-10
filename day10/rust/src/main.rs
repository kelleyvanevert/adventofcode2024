#![feature(let_chains)]

use std::time::Instant;

use fxhash::FxHashSet;
use itertools::Itertools;

fn main() {
    let input = include_str!("../../../inputs/input_10.txt");

    time(|| {
        // <1ms
        println!("First part: {}", solve(input));
    });

    time(|| {
        // <1ms
        println!("Bonus: {}", bonus(input));
    });
}

// macro_rules! vprintln {
//     ($c:expr, $($x:tt)*) => { if $c { println!($($x)*); } }
// }

fn time<F>(mut f: F)
where
    F: FnMut(),
{
    let t0 = Instant::now();
    f();
    println!("  took {:?}", t0.elapsed());
}

fn collect_trail_ends(
    grid: &Vec<Vec<usize>>,
    x: usize,
    y: usize,
    ends: &mut FxHashSet<(usize, usize)>,
) {
    if grid[y][x] == 9 {
        ends.insert((x, y));
    } else {
        if x > 0 && grid[y][x - 1] == grid[y][x] + 1 {
            collect_trail_ends(grid, x - 1, y, ends)
        }
        if y > 0 && grid[y - 1][x] == grid[y][x] + 1 {
            collect_trail_ends(grid, x, y - 1, ends)
        }
        if x < grid[0].len() - 1 && grid[y][x + 1] == grid[y][x] + 1 {
            collect_trail_ends(grid, x + 1, y, ends)
        }
        if y < grid.len() - 1 && grid[y + 1][x] == grid[y][x] + 1 {
            collect_trail_ends(grid, x, y + 1, ends)
        }
    }
}

fn solve(input: &str) -> usize {
    let grid = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();

    let h = grid.len();
    let w = grid[0].len();

    (0..h)
        .map(|y| {
            (0..w)
                .map(|x| {
                    if grid[y][x] != 0 {
                        0
                    } else {
                        let mut ends = FxHashSet::default();
                        collect_trail_ends(&grid, x, y, &mut ends);
                        ends.len()
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn collect_all_trails(
    grid: &Vec<Vec<usize>>,
    mut trail: Vec<(usize, usize)>,
    x: usize,
    y: usize,
    trails: &mut FxHashSet<Vec<(usize, usize)>>,
) {
    trail.push((x, y));

    if grid[y][x] == 9 {
        trails.insert(trail);
    } else {
        if x > 0 && grid[y][x - 1] == grid[y][x] + 1 {
            collect_all_trails(grid, trail.clone(), x - 1, y, trails)
        }
        if y > 0 && grid[y - 1][x] == grid[y][x] + 1 {
            collect_all_trails(grid, trail.clone(), x, y - 1, trails)
        }
        if x < grid[0].len() - 1 && grid[y][x + 1] == grid[y][x] + 1 {
            collect_all_trails(grid, trail.clone(), x + 1, y, trails)
        }
        if y < grid.len() - 1 && grid[y + 1][x] == grid[y][x] + 1 {
            collect_all_trails(grid, trail.clone(), x, y + 1, trails)
        }
    }
}

fn bonus(input: &str) -> usize {
    let grid = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();

    let h = grid.len();
    let w = grid[0].len();

    (0..h)
        .map(|y| {
            (0..w)
                .map(|x| {
                    if grid[y][x] != 0 {
                        0
                    } else {
                        let mut trails = FxHashSet::default();
                        collect_all_trails(&grid, vec![], x, y, &mut trails);
                        trails.len()
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        ),
        36
    );

    assert_eq!(
        bonus(
            "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        ),
        81
    );
}
