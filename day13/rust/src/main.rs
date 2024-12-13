#![feature(let_chains)]

use itertools::Itertools;
use std::time::Instant;
use tuple::Map;

fn main() {
    let input = include_str!("../../../inputs/input_13.txt");

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

fn solve(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|problem| {
            let (btn_a, btn_b, prize) = problem.lines().collect_tuple().unwrap();
            let (ax, ay) = btn_a[10..].split_once(", ").unwrap();
            let (bx, by) = btn_b[10..].split_once(", ").unwrap();
            let (px, py) = prize[7..].split_once(", ").unwrap();
            (ax, ay, bx, by, px, py).map(|s| s[2..].parse::<i64>().unwrap())
        })
        .filter_map(|(ax, ay, bx, by, px, py)| {
            let wt = px * ay - py * ax;
            let wb = bx * ay - by * ax;
            if wt % wb == 0 {
                let w = wt / wb;
                let vt = px - w * bx;
                if vt % ax == 0 {
                    let v = vt / ax;
                    return Some(w + 3 * v);
                }
            }

            None
        })
        .sum::<i64>() as usize
}

fn bonus(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|problem| {
            let (btn_a, btn_b, prize) = problem.lines().collect_tuple().unwrap();
            let (ax, ay) = btn_a[10..].split_once(", ").unwrap();
            let (bx, by) = btn_b[10..].split_once(", ").unwrap();
            let (px, py) = prize[7..].split_once(", ").unwrap();
            (ax, ay, bx, by, px, py).map(|s| s[2..].parse::<i64>().unwrap())
        })
        .filter_map(|(ax, ay, bx, by, px, py)| {
            let px = px + 10000000000000;
            let py = py + 10000000000000;
            let wt = px * ay - py * ax;
            let wb = bx * ay - by * ax;
            if wt % wb == 0 {
                let w = wt / wb;
                let vt = px - w * bx;
                if vt % ax == 0 {
                    let v = vt / ax;
                    return Some(w + 3 * v);
                }
            }

            None
        })
        .sum::<i64>() as usize
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
",
        ),
        480
    );
}
