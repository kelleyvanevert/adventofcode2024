use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use std::{cmp::Ordering, collections::BinaryHeap, time::Instant};

fn main() {
    let input = include_str!("../../../inputs/input_16.txt");

    time(|| {
        // ±5ms
        println!("First part: {}", solve(input));
    });

    time(|| {
        // ±3ms
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

type Pos = (i32, i32);
type Dir = (i32, i32);
type State = (Pos, Dir);

fn solve(input: &str) -> usize {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Path(Pos, Dir, usize);

    // lower score is better
    impl Ord for Path {
        fn cmp(&self, other: &Self) -> Ordering {
            return other.2.cmp(&self.2);
        }
    }

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut grid = input
        .trim()
        .lines()
        .map(str::chars)
        .map(Itertools::collect_vec)
        .collect_vec();

    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    macro_rules! at {
        ($v:expr) => {
            grid[$v.1 as usize][$v.0 as usize]
        };
    }

    let reindeer = (1, h - 2);
    let ending = (w - 2, 1);

    at!(ending) = '.';
    at!(reindeer) = '.';

    // best-first search:
    // - keep a list of paths (= beam), from best to worst, each with: (pos, dir, score)
    // - keep popping the best off, and adding next steps
    // - (we don't need to worry about recursive paths, because of their horrible score)

    let mut reached = FxHashMap::default();

    let mut best = BinaryHeap::new();
    best.push(Path(reindeer, (1, 0), 0));

    while let Some(Path((x, y), (dx, dy), score)) = best.pop() {
        if let Some(&s) = reached.get(&((x, y), (dx, dy))) {
            if score >= s {
                // we've already seen a better score for that (pos+dir), so skip this branch
                continue;
            }
        }

        reached.insert(((x, y), (dx, dy)), score);

        if (x, y) == ending {
            return score as usize;
        }

        // counterclockwise
        // (1, 0) -> (0, -1) -> (-1, 0) -> (0, 1)
        best.push(Path((x, y), (dy, -dx), score + 1000));

        // clockwise
        // (1, 0) -> (0, 1) -> (-1, 0) -> (0, -1)
        best.push(Path((x, y), (-dy, dx), score + 1000));

        // walk forward
        if at!((x + dx, y + dy)) == '.' {
            best.push(Path((x + dx, y + dy), (dx, dy), score + 1));
        }
    }

    unreachable!("there's no path that leads to the ending")
}

fn bonus(input: &str) -> usize {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Path(FxHashSet<State>, Pos, Dir, usize);

    // lower score is better
    impl Ord for Path {
        fn cmp(&self, other: &Self) -> Ordering {
            return other.3.cmp(&self.3);
        }
    }

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let max = solve(input);

    println!("max = {max}");

    let mut grid = input
        .trim()
        .lines()
        .map(str::chars)
        .map(Itertools::collect_vec)
        .collect_vec();

    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    macro_rules! at {
        ($v:expr) => {
            grid[$v.1 as usize][$v.0 as usize]
        };
    }

    let reindeer = (1, h - 2);
    let ending = (w - 2, 1);

    at!(ending) = '.';
    at!(reindeer) = '.';

    let mut total_found = 0;
    loop {
        let mut found = 0;

        for y in 1..(h - 2) {
            for x in 1..(w - 2) {
                if at!((x, y)) == '.'
                    && [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                        .into_iter()
                        .filter(|p| at!(p) == '#')
                        .count()
                        >= 3
                {
                    at!((x, y)) = '#';
                    found += 1;
                }
            }
        }
        println!("loop {found}");

        total_found += found;
        if found == 0 {
            break;
        }
    }

    println!(
        "DONE {total_found} / {}",
        grid.iter()
            .map(|line| line.iter().filter(|&&c| c == '.').count())
            .sum::<usize>()
    );

    // best-first search:
    // - keep a list of paths (= beam), from best to worst, each with: (pos, dir, score)
    // - keep popping the best off, and adding next steps
    // - (we don't need to worry about recursive paths, because of their horrible score)

    let mut reached = FxHashMap::default();
    let mut eligible = FxHashSet::default();

    let mut best = BinaryHeap::new();
    best.push(Path(FxHashSet::default(), reindeer, (1, 0), 0));

    while let Some(Path(prev, (x, y), (dx, dy), score)) = best.pop() {
        if prev.contains(&((x, y), (dx, dy))) || prev.contains(&((x, y), (-dx, -dy))) {
            continue;
        }

        let mut curr = prev.clone();
        curr.insert(((x, y), (dx, dy)));

        if score > max {
            continue;
        }

        reached.insert(((x, y), (dx, dy)), score);

        if (x, y) == ending {
            // this is an end-path
            for p in curr {
                eligible.insert(p);
            }
            continue;
        }

        // counterclockwise
        // (1, 0) -> (0, -1) -> (-1, 0) -> (0, 1)
        best.push(Path(curr.clone(), (x, y), (dy, -dx), score + 1000));

        // clockwise
        // (1, 0) -> (0, 1) -> (-1, 0) -> (0, -1)
        best.push(Path(curr.clone(), (x, y), (-dy, dx), score + 1000));

        // walk forward
        if at!((x + dx, y + dy)) == '.' {
            best.push(Path(curr.clone(), (x + dx, y + dy), (dx, dy), score + 1));
        }
    }

    eligible.into_iter().map(|(pos, _)| pos).unique().count()
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"
        ),
        7036
    );

    assert_eq!(
        solve(
            "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"
        ),
        11048
    );

    assert_eq!(
        bonus(
            "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"
        ),
        45
    );

    assert_eq!(
        bonus(
            "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"
        ),
        64
    );
}
