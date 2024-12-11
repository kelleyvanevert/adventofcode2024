#![feature(let_chains)]

use cached::proc_macro::cached;
use std::time::Instant;
use tuple::Map;

fn main() {
    let input = include_str!("../../../inputs/input_11.txt");

    time(|| {
        // <1ms
        println!("First part: {}", solve(input));
    });

    time(|| {
        // ±20ms
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

#[cached]
fn splits(n: usize, steps: usize) -> usize {
    if steps == 0 {
        return 1;
    }

    // case 1
    if n == 0 {
        return splits(1, steps - 1);
    }

    // case 2
    let s = format!("{n}");
    if s.len() % 2 == 0 {
        let (le, ri) = s.split_at(s.len() / 2).map(|s| s.parse::<usize>().unwrap());
        return splits(le, steps - 1) + splits(ri, steps - 1);
    }

    // case 3
    splits(n * 2024, steps - 1)
}

fn solve(input: &str) -> usize {
    input
        .trim()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .map(|n| splits(n, 25))
        .sum()
}

fn bonus(input: &str) -> usize {
    input
        .trim()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .map(|n| splits(n, 75))
        .sum()
}

#[test]
fn test() {
    assert_eq!(solve("125 17",), 55312);
}
