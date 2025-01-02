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
        // ±200ms
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

    let mut edges: FxHashSet<(Pos, Pos)> = FxHashSet::default();

    for x in (1..w).step_by(2) {
        for y in (1..h).step_by(2) {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let n = (x + 2 * dx, y + 2 * dy);
                if at!((x + dx, y + dy)) == '.' {
                    // println!(
                    //     "     edge from {:?} to {:?} because {:?} =.",
                    //     (x, y),
                    //     n,
                    //     (x + dx, y + dy)
                    // );
                    edges.insert(((x, y), n));
                    edges.insert((n, (x, y)));
                } else {
                    // println!(
                    //     "  NO edge from {:?} to {:?} because {:?} !=.",
                    //     (x, y),
                    //     n,
                    //     (x + dx, y + dy)
                    // );
                }
            }
        }
    }

    // ==

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Path {
        pos: Pos,
        dir: Dir,
        path: Vec<State>,
        cost: usize,
    }

    impl Path {
        fn turn_counterclockwise(&self) -> Option<Path> {
            let pos = self.pos;

            let (dx, dy) = self.dir;
            let dir = (dy, -dx);

            if self.path.contains(&(pos, dir)) {
                None
            } else {
                let mut path = self.path.clone();
                path.push((pos, dir));

                let cost = self.cost + 1000;

                Some(Path {
                    pos,
                    dir,
                    path,
                    cost,
                })
            }
        }

        fn turn_clockwise(&self) -> Option<Path> {
            let pos = self.pos;

            let (dx, dy) = self.dir;
            let dir = (-dy, dx);

            if self.path.contains(&(pos, dir)) {
                None
            } else {
                let mut path = self.path.clone();
                path.push((pos, dir));

                let cost = self.cost + 1000;

                Some(Path {
                    pos,
                    dir,
                    path,
                    cost,
                })
            }
        }

        fn walk_forward(&self, edges: &FxHashSet<(Pos, Pos)>) -> Option<Path> {
            let (x, y) = self.pos;
            let (dx, dy) = self.dir;
            let pos = (x + dx * 2, y + dy * 2);

            let dir = self.dir;

            if self.path.contains(&(pos, dir)) || !edges.contains(&(self.pos, pos)) {
                None
            } else {
                let mut path = self.path.clone();
                path.push(((x + dx, y + dy), dir));
                path.push((pos, dir));

                let cost = self.cost + 2;

                Some(Path {
                    pos,
                    dir,
                    path,
                    cost,
                })
            }
        }
    }

    // lower score is better
    impl Ord for Path {
        fn cmp(&self, other: &Self) -> Ordering {
            return other.cost.cmp(&self.cost);
        }
    }

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut queue = BinaryHeap::new();
    queue.push(Path {
        pos: reindeer,
        dir: (1, 0),
        path: vec![(reindeer, (1, 0))],
        cost: 0,
    });

    let mut lowest_cost_paths = vec![];
    let mut lowest_cost = usize::MAX;

    let mut reached = FxHashMap::default();

    while let Some(path) = queue.pop() {
        if let Some(&s) = reached.get(&(path.pos, path.dir)) {
            if path.cost > s {
                // we've already seen a better score for that (pos+dir), so skip this branch
                continue;
            }
        }

        reached.insert((path.pos, path.dir), path.cost);

        if path.cost > lowest_cost {
            // we're done, because all next paths from the queue will have a higher cost yet
            break;
        }

        if path.pos == ending {
            lowest_cost = path.cost;
            lowest_cost_paths.push(path);
            continue;
        }

        // explore neighbors
        for next in [
            path.turn_counterclockwise(),
            path.turn_clockwise(),
            path.walk_forward(&edges),
        ] {
            if let Some(next_path) = next {
                queue.push(next_path);
            }
        }
    }

    let visited = lowest_cost_paths
        .into_iter()
        .flat_map(|path| path.path.into_iter().map(|(pos, _)| pos))
        .collect::<FxHashSet<_>>();

    visited.len()
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
