use cached::proc_macro::cached;
use std::time::Instant;

fn main() {
    let input = include_str!("../../../inputs/input_22.txt");

    time(|| {
        // ±500ms
        println!("First part: {}", solve(input));
    });

    // time(|| {
    //     // ..
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

#[cached]
fn next_secret(secret: usize) -> usize {
    let secret = ((secret * 64) ^ secret) % 16777216;
    let secret = ((secret / 32) ^ secret) % 16777216;
    let secret = ((secret * 2048) ^ secret) % 16777216;
    secret
}

fn apply(f: fn(usize) -> usize, n: usize, mut x: usize) -> usize {
    for _ in 0..n {
        x = f(x);
    }

    x
}

fn solve(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let secret = line.parse::<usize>().unwrap();
            let result = apply(next_secret, 2000, secret);
            // println!("{secret}: {result}");
            result
        })
        .sum()
}

fn bonus(input: &str) -> usize {
    0
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
1
10
100
2024
"
        ),
        37327623
    );

    //     assert_eq!(
    //         bonus(
    //             "
    // r, wr, b, g, bwu, rb, gb, br

    // brwrr
    // bggr
    // gbbr
    // rrbgbr
    // ubwu
    // bwurrg
    // brgr
    // bbrgwb
    // "
    //         ),
    //         16
    //     );
}
