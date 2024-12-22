use cached::proc_macro::cached;
use fxhash::FxHashSet;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{cmp::Reverse, time::Instant};

fn main() {
    let input = include_str!("../../../inputs/input_19.txt");

    time(|| {
        // ±40ms (using parallelization)
        println!("First part: {}", solve(input));
    });

    time(|| {
        // ±300ms (using parallelization)
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

/**
 * Checks whether we can build the pattern out of the towels
 *
 * The trick is to avoid a breadth or depth first search, because that takes way too long, and instead of find a way to heuristically get the hardest part out the way first.
 *
 * The way in which we do that is by first tallying up, for each index i of the pattern, how many towels could fit on top of that index (not starting at, but covering). Then we find the index with the lowest amount, and split the pattern there, and recurse. This way, we find a disproof as soon as possible.
 */
fn is_possible(towels: &FxHashSet<&str>, pattern: &str, depth: usize) -> bool {
    if pattern.len() == 0 {
        return true;
    }

    let mut count = vec![vec![]; pattern.len()];

    for &towel in towels {
        if towel.len() > pattern.len() {
            continue;
        }

        for i in 0..(pattern.len() - towel.len() + 1) {
            if pattern[i..].starts_with(towel) {
                for j in 0..towel.len() {
                    count[i + j].push((i, towel));
                }
            }
        }
    }

    let mut options = count.into_iter().min_by_key(|t| t.len()).unwrap();

    if options.len() == 0 {
        return false;
    }

    options.sort_by_key(|o| Reverse(o.1.len()));

    options.into_iter().any(|(i, towel)| {
        let le = &pattern[0..i];
        let ri = &pattern[(i + towel.len())..];
        is_possible(towels, le, depth + 1) && is_possible(towels, ri, depth + 1)
    })
}

fn solve(input: &str) -> usize {
    let (towels, patterns) = input.trim().split_once("\n\n").unwrap();
    let towels: FxHashSet<&str> = towels.split(", ").collect();
    let patterns = patterns.lines().collect_vec();

    patterns
        .into_par_iter()
        .filter(|pattern| is_possible(&towels, pattern, 0))
        .count()
}

/**
 * The trick here is that generalizing the previous solution is actually possible, because it's performant enough IF we use dynamic programming. And that's super easily done using the `#[cached]` macro :)
 *
 * (I'm skipping the `towels` argument in the cache, which of course is only possible because I know we'll only be using this code for 1 input set.)
 */
#[cached(key = "String", convert = r#"{ pattern.into() }"#)]
fn is_possible_ways(towels: &FxHashSet<&str>, pattern: &str, depth: usize) -> usize {
    if pattern.len() == 0 {
        return 1;
    }

    let mut count = vec![vec![]; pattern.len()];

    for &towel in towels {
        if towel.len() > pattern.len() {
            continue;
        }

        for i in 0..(pattern.len() - towel.len() + 1) {
            if pattern[i..].starts_with(towel) {
                for j in 0..towel.len() {
                    count[i + j].push((i, towel));
                }
            }
        }
    }

    let mut options = count.into_iter().min_by_key(|t| t.len()).unwrap();

    if options.len() == 0 {
        return 0;
    }

    options.sort_by_key(|o| Reverse(o.1.len()));

    options
        .into_iter()
        .map(|(i, towel)| {
            let le = &pattern[0..i];
            let ri = &pattern[(i + towel.len())..];
            is_possible_ways(towels, le, depth + 1) * is_possible_ways(towels, ri, depth + 1)
        })
        .sum()
}

fn bonus(input: &str) -> usize {
    let (towels, patterns) = input.trim().split_once("\n\n").unwrap();
    let towels: FxHashSet<&str> = towels.split(", ").collect();
    let patterns = patterns.lines().collect_vec();

    patterns
        .into_par_iter()
        .map(|pattern| is_possible_ways(&towels, pattern, 0))
        .sum()
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"
        ),
        6
    );

    assert_eq!(
        bonus(
            "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"
        ),
        16
    );
}
