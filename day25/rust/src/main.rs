use itertools::Itertools;
use std::time::Instant;

fn main() {
    let input = include_str!("../../../inputs/input_25.txt");

    time(|| {
        // <1ms
        println!("First part: {}", solve(input));
    });

    // time(|| {
    //     // ±3ms
    //     println!("Bonus: {}", bonus_smart(input));
    // });
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

fn solve(input: &str) -> usize {
    let mut keys = vec![];
    let mut locks = vec![];

    let w = 5;
    let h = 7;

    for item in input.trim().split("\n\n") {
        let grid = item
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        if grid[0][0] == '#' {
            // lock
            locks.push(
                (0..w)
                    .map(|c| (0..h).find(|&r| grid[r][c] == '.').unwrap() - 1)
                    .collect_vec(),
            );
        } else {
            // key
            keys.push(
                (0..w)
                    .map(|c| 6 - (0..h).find(|&r| grid[r][c] == '#').unwrap())
                    .collect_vec(),
            );
        }
    }

    let mut fit = 0;

    for key in &keys {
        'iter: for lock in &locks {
            for i in 0..w {
                if key[i] + lock[i] > 5 {
                    continue 'iter;
                }
            }

            fit += 1;
        }
    }

    fit
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"
        ),
        3
    );
}
