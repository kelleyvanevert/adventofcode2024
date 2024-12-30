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
        // ±2s
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

fn find_triples(n: usize, adj: &FxHashSet<(usize, usize)>) -> FxHashSet<Vec<usize>> {
    let mut groups = FxHashSet::default();

    for i in 0..n {
        for j in 0..n {
            if adj.contains(&(i, j)) {
                for k in (j + 1)..n {
                    if adj.contains(&(i, k)) && adj.contains(&(j, k)) {
                        let mut group = vec![i, j, k];
                        group.sort();
                        groups.insert(group);
                    }
                }
            }
        }
    }

    groups
}

fn bonus(input: &str) -> String {
    // collect computer names in whatever order, so we can work with indices as names afterwards
    // ===

    let mut computers = input
        .trim()
        .lines()
        .map(|line| line.split_once("-").unwrap().0)
        .collect_vec();

    computers.sort();

    let n = computers.len();

    // build adjacency-matrix (between indices i,j)
    // ===

    let mut adj = FxHashSet::default();

    for line in input.trim().lines() {
        let (a, b) = line.split_once("-").unwrap();
        let ai = index_of(&computers, &a).unwrap();
        let bi = index_of(&computers, &b).unwrap();
        for (i, j) in [(ai, bi), (bi, ai)] {
            adj.insert((i, j));
            adj.insert((j, i));
        }
    }

    let mut groups = find_triples(n, &adj);

    // then, iteratively extend groups where possible to size 4, 5, etc.. until only one group remains
    let mut size = 3;
    let mut any_largest = groups.iter().next().unwrap().clone();
    loop {
        size += 1;
        println!("Extending groups to size {size}...");

        groups = groups
            .into_par_iter()
            .flat_map(|group| {
                let mut expanded = vec![];
                for i in 0..n {
                    if !group.contains(&i) && group.iter().all(|&j| adj.contains(&(i, j))) {
                        let mut group = group.clone();
                        group.push(i);
                        group.sort();
                        expanded.push(group);
                    }
                }
                expanded
            })
            .collect::<FxHashSet<_>>();

        if groups.len() > 0 {
            any_largest = groups.iter().next().unwrap().clone();
        }

        println!("  -> now there's {}", groups.len());
        if groups.len() <= 1 {
            break;
        }
    }

    any_largest.into_iter().map(|i| computers[i]).join(",")
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
