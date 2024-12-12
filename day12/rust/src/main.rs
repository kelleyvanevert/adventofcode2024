#![feature(let_chains)]

use fxhash::FxHashSet;
use itertools::Itertools;
use std::time::Instant;

fn main() {
    let input = include_str!("../../../inputs/input_12.txt");

    time(|| {
        // ±4ms
        println!("First part: {}", solve(input));
    });

    time(|| {
        // ±6ms
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

fn solve(input: &str) -> usize {
    let grid = input
        .trim()
        .lines()
        .map(str::chars)
        .map(Itertools::collect_vec)
        .collect_vec();

    let h = grid.len();
    let w = grid[0].len();

    let mut seen = FxHashSet::default();
    let mut total_price = 0;

    for y in 0..h {
        for x in 0..w {
            if seen.contains(&(x, y)) {
                continue;
            }

            // 1. Gather region
            let plant = grid[y][x];
            let mut todo = vec![(x, y)];
            let mut region = FxHashSet::default();
            while let Some((x, y)) = todo.pop() {
                region.insert((x, y));
                seen.insert((x, y));
                if x > 0 && grid[y][x - 1] == plant && !region.contains(&(x - 1, y)) {
                    todo.push((x - 1, y));
                }
                if y > 0 && grid[y - 1][x] == plant && !region.contains(&(x, y - 1)) {
                    todo.push((x, y - 1));
                }
                if x < w - 1 && grid[y][x + 1] == plant && !region.contains(&(x + 1, y)) {
                    todo.push((x + 1, y));
                }
                if y < h - 1 && grid[y + 1][x] == plant && !region.contains(&(x, y + 1)) {
                    todo.push((x, y + 1));
                }
            }

            // 2. Gather perimeter
            let mut perimeter = FxHashSet::default();
            for &(x, y) in region.iter() {
                for side in [(x, y, '|'), (x, y, '-'), (x + 1, y, '|'), (x, y + 1, '-')] {
                    if perimeter.contains(&side) {
                        perimeter.remove(&side);
                    } else {
                        perimeter.insert(side);
                    }
                }
            }

            // 3. Add price
            total_price += region.len() * perimeter.len();
        }
    }

    total_price
}

fn bonus(input: &str) -> usize {
    let grid = input
        .trim()
        .lines()
        .map(str::chars)
        .map(Itertools::collect_vec)
        .collect_vec();

    let h = grid.len();
    let w = grid[0].len();

    let mut seen = FxHashSet::default();
    let mut total_price = 0;

    fn d(a: usize, b: usize) -> usize {
        if a > b {
            a - b
        } else {
            b - a
        }
    }

    let siding = |plant: char,
                  (x1, y1, o1): (usize, usize, char),
                  (x2, y2, o2): (usize, usize, char)|
     -> bool {
        (
            // This is the "actual check", which reads normally:
            // Two edges are siding if they're the same orientation
            //  and next to each other in that orientation
               o1 == o2 && o1 == '|' && x1 == x2 && d(y1, y2) == 1
            || o1 == o2 && o1 == '-' && y1 == y2 && d(x1, x2) == 1
        ) &&
            // But we also add this quick-fix to solve the problem that occurs in the map below:
            //
            //   AAAAAA
            //   AAABBA
            //   AAABBA
            //   ABBAAA
            //   ABBAAA
            //   AAAAAA
            //
            // Here, we don't want the top-right B region's left side to connect
            //  to the bottom-left B region's right side. But they're both vertical
            //  and will align with the condition above. So, quick-fix it using the
            //  knowledge that, whether the coordinates point inside or outside of
            //  the region (because that can differ with our chosen encoding), at least
            //  both lots should either BOTH be the right plant type, or BOTH be another.
            //  This condition will hold for the two Bs above at (3,1) and (3,2),
            //  as well as for the two As below at (3,3) and (3,4), but critically not
            //  for the B and A at (3,2) and (3,3).
            (y1 >= h
                || x1 >= w
                || y2 >= h
                || x2 >= w
                || ((grid[y1][x1] == plant) == (grid[y2][x2] == plant)))
    };

    for y in 0..h {
        for x in 0..w {
            if seen.contains(&(x, y)) {
                continue;
            }

            // 1. Gather region
            let plant = grid[y][x];
            let mut todo = vec![(x, y)];
            let mut region = FxHashSet::default();
            while let Some((x, y)) = todo.pop() {
                region.insert((x, y));
                seen.insert((x, y));
                if x > 0 && grid[y][x - 1] == plant && !region.contains(&(x - 1, y)) {
                    todo.push((x - 1, y));
                }
                if y > 0 && grid[y - 1][x] == plant && !region.contains(&(x, y - 1)) {
                    todo.push((x, y - 1));
                }
                if x < w - 1 && grid[y][x + 1] == plant && !region.contains(&(x + 1, y)) {
                    todo.push((x + 1, y));
                }
                if y < h - 1 && grid[y + 1][x] == plant && !region.contains(&(x, y + 1)) {
                    todo.push((x, y + 1));
                }
            }

            // 2. Gather perimeter
            let mut perimeter = FxHashSet::default();
            for &(x, y) in region.iter() {
                for side in [(x, y, '|'), (x, y, '-'), (x + 1, y, '|'), (x, y + 1, '-')] {
                    if perimeter.contains(&side) {
                        perimeter.remove(&side);
                    } else {
                        perimeter.insert(side);
                    }
                }
            }

            // 3. Simplify perimeter -> sides
            let mut sides: Vec<Vec<(usize, usize, char)>> = vec![];
            let mut perimeter = perimeter.drain().collect_vec();
            while let Some(edge) = perimeter.pop() {
                let mut side = vec![edge];

                while let Some(i) = perimeter
                    .iter()
                    .position(|&edge2| side.iter().any(|&e| siding(plant, e, edge2)))
                {
                    side.push(perimeter.remove(i));
                }

                sides.push(side);
            }

            // 4. Add price
            total_price += region.len() * sides.len();
        }
    }

    total_price
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
        ),
        1930
    );

    assert_eq!(
        bonus(
            "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
        ),
        1206
    );

    assert_eq!(
        bonus(
            "
AAAA
BBCD
BBCC
EEEC
",
        ),
        80
    );

    assert_eq!(
        bonus(
            "
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
",
        ),
        236
    );

    assert_eq!(
        bonus(
            "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
",
        ),
        368
    );
}
