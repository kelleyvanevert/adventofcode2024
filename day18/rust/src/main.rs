use binary_heap_plus::*;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use std::{cmp::Reverse, time::Instant};
use tuple::Map;

fn main() {
    let input = include_str!("../../../inputs/input_18.txt");

    time(|| {
        // <1ms
        println!("First part: {}", solve(71, 1024, input));
    });

    // time(|| {
    //     // ±3ms
    //     println!("Bonus: {}", bonus(input));
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

type Pos = (i32, i32);

fn solve(dim: i32, num_corrupted: usize, input: &str) -> usize {
    let blocked = input
        .trim()
        .lines()
        .take(num_corrupted)
        .map(|line| {
            line.split_once(",")
                .unwrap()
                .map(|s| s.parse::<i32>().unwrap())
        })
        .collect::<FxHashSet<_>>();

    let start = (0, 0);
    let end = (dim - 1, dim - 1);

    // pos -> min num steps needed to get there
    let mut reached = FxHashMap::default();

    let mut queue = BinaryHeap::new_by_key(|t: &(Pos, usize)| Reverse(t.1));
    queue.push((start, 0));

    while let Some(((x, y), cost)) = queue.pop() {
        if let Some(min_cost) = reached.get(&(x, y)) {
            if cost >= *min_cost {
                // already found a path to this pos for less cost => skip branch
                continue;
            }
        }

        reached.insert((x, y), cost);

        if (x, y) == end {
            return cost;
        }

        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if nx >= 0 && ny >= 0 && nx < dim && ny < dim {
                if !blocked.contains(&(nx, ny)) {
                    queue.push(((nx, ny), cost + 1));
                }
            }
        }
    }

    unreachable!("algorithm should already have found a path")
}

fn bonus(input: &str) -> usize {
    0
}

#[test]
fn test() {
    assert_eq!(
        solve(
            7,
            12,
            "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"
        ),
        22
    );
}
