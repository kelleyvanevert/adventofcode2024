use fxhash::FxHashMap;
use itertools::Itertools;
use std::time::Instant;

fn main() {
    let input = include_str!("../../../inputs/input_20.txt");

    time(|| {
        // ±30ms
        println!("First part: {}", solve(2, 100, input));
    });

    time(|| {
        // ±30ms
        println!("Bonus: {}", solve(20, 100, input));
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

type Pos = (i32, i32);

fn manhattan((ax, ay): Pos, (bx, by): Pos) -> i32 {
    (bx - ax).abs() + (by - ay).abs()
}

fn solve(max_cheat_len: i32, at_least: i32, input: &str) -> usize {
    let mut grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    macro_rules! at {
        ($c:expr) => {
            grid[$c.1 as usize][$c.0 as usize]
        };
    }

    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let start = (0..h)
        .find_map(|y| (0..w).find_map(|x| (at!((x, y)) == 'S').then_some((x, y))))
        .unwrap();

    let end = (0..h)
        .find_map(|y| (0..w).find_map(|x| (at!((x, y)) == 'E').then_some((x, y))))
        .unwrap();

    at!(start) = '.';
    at!(end) = '.';

    let mut route = vec![start];
    let mut prev = (-1, -1);
    let (mut x, mut y) = start;
    while (x, y) != end {
        let n = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
            .into_iter()
            .find(|&n| at!(n) == '.' && prev != n)
            .unwrap();

        prev = (x, y);
        (x, y) = n;
        route.push(n);
    }

    // let mut saving = FxHashMap::default();
    let mut saving_at_least = 0;

    for i in 0..(route.len() as i32) {
        for j in (i + 3)..(route.len() as i32) {
            let d = manhattan(route[i as usize], route[j as usize]);
            if d <= max_cheat_len {
                let save = j - i - d;
                if save >= at_least {
                    saving_at_least += 1;
                    // saving.entry(save).and_modify(|n| *n += 1).or_insert(1);
                    // println!("shortcut ({d}) {i} -> {j} (saving {})", save);
                }
            }
        }
    }

    // for k in saving.keys().sorted() {
    //     println!(
    //         "  there's {} cheats saving {} picos",
    //         saving.get(k).unwrap(),
    //         k
    //     );
    // }

    saving_at_least
}

#[test]
fn test() {
    assert_eq!(
        solve(
            2,
            0,
            "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"
        ),
        44
    );

    assert_eq!(
        solve(
            20,
            50,
            "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"
        ),
        285
    );
}
