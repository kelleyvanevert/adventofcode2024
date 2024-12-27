use cached::proc_macro::cached;
use fxhash::FxHashMap;
use itertools::Itertools;
use std::time::Instant;

fn main() {
    let input = include_str!("../../../inputs/input_22.txt");

    time(|| {
        // ±500ms
        println!("First part: {}", solve(input));
    });

    time(|| {
        // ±1s
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
fn next_secret(secret: usize) -> usize {
    let secret = ((secret << 06) ^ secret) % 0b1000000000000000000000000;
    let secret = ((secret >> 05) ^ secret) % 0b1000000000000000000000000;
    let secret = ((secret << 11) ^ secret) % 0b1000000000000000000000000;
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
            apply(next_secret, 2000, secret)
        })
        .sum()
}

fn bonus(input: &str) -> usize {
    let initial_secrets = input
        .trim()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect_vec();

    let infos = initial_secrets
        .iter()
        .map(|&secret| {
            // println!("Secret {secret}");

            let n = 2000;
            let secret = secret; // 123

            let mut seq = vec![0; n + 1];
            seq[0] = secret;
            for i in 0..n {
                seq[i + 1] = next_secret(seq[i]);
            }

            // println!("  seq: {:?}", seq);

            let diffseq = seq
                .iter()
                .tuple_windows()
                .map(|(a, b)| (b % 10) as i32 - (a % 10) as i32)
                .collect_vec();

            // println!("  diffs: {:?}", diffseq);

            let mut found = FxHashMap::default();

            for (i, (&a, &b, &c, &d)) in diffseq.iter().tuple_windows().enumerate() {
                let t = (a, b, c, d);
                let price = seq[i + 4] % 10;
                // println!("    t = {t:?} -> {price}");

                if !found.contains_key(&t) {
                    found.insert(t, price);
                }
            }

            // println!("  has? {:?}", found.get(&t));

            (seq, diffseq, found)
        })
        .collect_vec();

    let mut totals = FxHashMap::default();

    for (_, _, found) in &infos {
        for (&t, &price) in found {
            totals
                .entry(t)
                .and_modify(|total| {
                    *total += price;
                })
                .or_insert(price);
        }
    }

    totals.values().cloned().max().unwrap()
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

    assert_eq!(
        bonus(
            "
1
2
3
2024
"
        ),
        23
    );
}
