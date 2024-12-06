#![feature(let_chains)]

use fxhash::FxHashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("../../../inputs/input_06.txt");

    time(|| {
        // <1ms
        println!("First part: {}", solve(input));
    });

    // time(|| {
    //     // ±1.5s
    //     println!("Bonus: {}", bonus(input));
    // });

    time(|| {
        // ±400ms
        println!("Bonus: {}", bonus_v2(input));
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

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, c)| match c {
                '^' => Some((x as i32, y as i32)),
                _ => None,
            })
        })
        .unwrap();

    let (mut dx, mut dy) = (0, -1);
    let mut seen = FxHashSet::default();

    let (mut x, mut y) = start;
    loop {
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || nx >= w || ny < 0 || ny >= h {
            break;
        }

        if grid[ny as usize][nx as usize] == '#' {
            (dx, dy) = (0 - dy, dx)
        } else {
            (x, y) = (nx, ny);

            seen.insert((x, y));
        }
    }

    seen.len()
}

fn bonus(input: &str) -> usize {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, c)| match c {
                '^' => Some((x as i32, y as i32)),
                _ => None,
            })
        })
        .unwrap();

    let mut num_loops = 0;

    for oy in 0..h {
        println!("At y = {oy} ({num_loops} loops so far) ...");
        'find: for ox in 0..w {
            if grid[oy as usize][ox as usize] != '.' {
                continue 'find;
            }

            let mut grid = grid.clone();
            grid[oy as usize][ox as usize] = '#';

            // execute...

            let (mut x, mut y) = start;

            let (mut dx, mut dy) = (0, -1);
            let mut been = FxHashSet::default();
            been.insert((x, y, dx, dy));

            loop {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || nx >= w || ny < 0 || ny >= h {
                    continue 'find;
                }

                if grid[ny as usize][nx as usize] == '#' {
                    (dx, dy) = (0 - dy, dx)
                } else {
                    (x, y) = (nx, ny);
                }

                if been.contains(&(x, y, dx, dy)) {
                    num_loops += 1;
                    continue 'find;
                }

                been.insert((x, y, dx, dy));
            }
        }
    }

    num_loops as usize
}

fn bonus_v2(input: &str) -> usize {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, c)| match c {
                '^' => Some((x as i32, y as i32)),
                _ => None,
            })
        })
        .unwrap();

    let check_if_loop = |obstruction: (i32, i32)| {
        let (mut x, mut y) = start;
        // !! `clone` is necessary due to a bug in AL that's been in there for a long time, maybe even since the beginning...

        let (mut dx, mut dy) = (0, -1);

        let mut been = FxHashSet::default();
        been.insert((x, y, dx, dy));

        loop {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || nx >= w || ny < 0 || ny >= h {
                return false;
            }

            if grid[ny as usize][nx as usize] == '#' || (nx, ny) == obstruction {
                (dx, dy) = (0 - dy, dx)
            } else {
                (x, y) = (nx, ny);
            }

            if been.contains(&(x, y, dx, dy)) {
                return true;
            }

            been.insert((x, y, dx, dy));
        }
    };

    let mut loops_found = FxHashSet::default();

    let (mut dx, mut dy) = (0, -1);

    let (mut x, mut y) = start;
    loop {
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || nx >= w || ny < 0 || ny >= h {
            break;
        }

        if grid[ny as usize][nx as usize] == '#' {
            (dx, dy) = (0 - dy, dx)
        } else {
            (x, y) = (nx, ny);

            if !loops_found.contains(&(nx, ny)) && check_if_loop((nx, ny)) {
                loops_found.insert((nx, ny));
            }
        }
    }

    loops_found.len()
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
        ),
        41
    );

    assert_eq!(
        bonus(
            "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
        ),
        6
    );

    assert_eq!(
        bonus_v2(
            "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
        ),
        6
    );
}
