use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let input = include_str!("../../../inputs/input_23.txt");

    time(|| {
        // ±1.5ms
        println!("First part: {}", solve(input));
    });

    time(|| {
        // ±?
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
    let mut adj = FxHashMap::default();
    let mut computers = FxHashSet::default();

    for line in input.trim().lines() {
        let (a, b) = line.split_once("-").unwrap();
        for (a, b) in [(a, b), (b, a)] {
            computers.insert(a);
            adj.entry(a)
                .or_insert_with(|| FxHashSet::default())
                .insert(b);
        }
    }

    let mut groups = FxHashSet::default();

    for &a in &computers {
        if let Some(connected) = adj.get(a) {
            for (&b, &c) in connected.into_iter().tuple_combinations() {
                if adj.get(b).unwrap().contains(c)
                    && [a, b, c].into_iter().any(|a| a.starts_with("t"))
                {
                    let mut group = vec![a, b, c];
                    group.sort();
                    groups.insert(group);
                }
            }
        }
    }

    // for g in groups {
    //     println!("{g:?}");
    // }

    groups.len()
}

fn index_of<T: Eq>(vec: &Vec<T>, el: &T) -> Option<usize> {
    vec.iter().position(|e| e == el)
}

// n         : usize
// adj[i][j] : bool

fn bonus(input: &str) -> String {
    let computers = input
        .trim()
        .lines()
        .map(|line| line.split_once("-").unwrap().0)
        .collect_vec();

    let n = computers.len();

    let mut adj = vec![vec![false; n]; n];

    for line in input.trim().lines() {
        let (a, b) = line.split_once("-").unwrap();
        let ai = index_of(&computers, &a).unwrap();
        let bi = index_of(&computers, &b).unwrap();
        for (i, j) in [(ai, bi), (bi, ai)] {
            adj[i][j] = true;
            adj[j][i] = true;
        }
    }

    let mut groups = FxHashSet::default();

    for i in 0..n {
        for j in 0..n {
            if adj[i][j] {
                for k in (j + 1)..n {
                    if adj[i][k] && adj[j][k] {
                        let mut group = vec![i, j, k];
                        group.sort();
                        groups.insert(group);
                    }
                }
            }
        }
    }

    // let n2 = groups.len();

    // let mut adj2 = vec!

    "".into()
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"
        ),
        7
    );

    assert_eq!(
        bonus(
            "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"
        ),
        "co,de,ka,ta".to_owned()
    );
}
