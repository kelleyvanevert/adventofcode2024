use fxhash::FxHashMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::{iter::once, time::Instant};

fn main() {
    let input = include_str!("../../../inputs/input_21.txt");

    time(|| {
        // ±9s
        println!("First part: {}", solve(input));
    });

    // time(|| {
    //     // ±300ms (using parallelization)
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

lazy_static! {
    static ref DIRPAD: Graph = Graph::from("^>A, <>v, v>>, ^vv, Av>");
    static ref NUMPAD: Graph =
        Graph::from("7>8, 8>9, 4>5, 5>6, 1>2, 2>3, 0>A, 7v4, 4v1, 8v5, 5v2, 2v0, 9v6, 6v3, 3vA");
}

fn opposite_dir(dir: char) -> char {
    match dir {
        '^' => 'v',
        'v' => '^',
        '>' => '<',
        '<' => '>',
        _ => unreachable!("not a dir: {}", dir),
    }
}

type Path = Vec<char>;

#[derive(Debug)]
struct Graph {
    n: usize,
    vertices: Vec<char>,
    adjacent: FxHashMap<(char, char), char>,
    shortest: FxHashMap<(char, char), Vec<Path>>,
}

impl Graph {
    fn from(description: &'static str) -> Self {
        let edges = description
            .split(", ")
            .flat_map(|ab| {
                let (a, dir, b) = ab.chars().collect_tuple().unwrap();
                [(a, dir, b), (b, opposite_dir(dir), a)]
            })
            .collect_vec();

        let vertices = edges
            .iter()
            .flat_map(|&(a, _, b)| [a, b])
            .unique()
            .collect_vec();

        let n = vertices.len();

        let mut adjacent = FxHashMap::default();

        for &(a, dir, b) in edges.iter() {
            adjacent.insert((a, b), dir);
            // adjacent.insert((b, a), dir);
        }

        // Pre-compute, for every combo (a, b), all shortest paths
        // (This is very inefficient qua memory for big graphs, but in our case we know the graph is small)
        let shortest = {
            let mut shortest = FxHashMap::default();

            for &a in vertices.iter() {
                shortest.insert((a, a), (0, vec![vec![a]]));
            }

            for &(a, _, b) in edges.iter() {
                shortest.insert((a, b), (1, vec![vec![a, b]]));
                shortest.insert((b, a), (1, vec![vec![b, a]]));
            }

            loop {
                let mut found_new = false;

                let all_shortest_paths = shortest.values().flat_map(|t| t.1.clone()).collect_vec();
                let all_shortest_paths_by_start = all_shortest_paths
                    .iter()
                    .cloned()
                    .into_group_map_by(|p| p[0]);

                for path in all_shortest_paths {
                    let a = path[0];
                    let b = path[path.len() - 1];
                    for extension in all_shortest_paths_by_start.get(&b).unwrap() {
                        let mut extended = path.clone();
                        extended.extend(&extension[1..]);
                        let c = extended[extended.len() - 1];
                        let len = extended.len() - 1;

                        if !shortest.contains_key(&(a, c)) {
                            shortest.insert((a, c), (len, vec![extended.clone()]));
                            found_new = true;
                        } else {
                            let (min_len, paths) = shortest.get_mut(&(a, c)).unwrap();
                            if len == *min_len && !paths.contains(&extended) {
                                paths.push(extended);
                                found_new = true;
                            } else if len < *min_len {
                                *min_len = len;
                                *paths = vec![extended];
                                found_new = true;
                            }
                        }
                    }
                }

                if !found_new {
                    break;
                }
            }

            shortest
                .into_iter()
                .map(|((a, b), (_min_len, paths))| ((a, b), paths))
                .collect()
        };

        Graph {
            n,
            vertices,
            adjacent,
            shortest,
        }
    }

    fn shortest_paths(&self, start: char, end: char) -> Vec<Path> {
        self.shortest.get(&(start, end)).unwrap().clone()
    }

    fn move_to_and_press(&self, path: &Path) -> Path {
        path.iter()
            .tuple_windows()
            .map(|(a, b)| *self.adjacent.get(&(*a, *b)).unwrap())
            .chain(once('A'))
            .collect_vec()
    }

    fn shortest_path_dirs_along(&self, along: Vec<char>) -> Vec<Path> {
        let h = along
            .windows(2)
            .map(|w| {
                self.shortest_paths(w[0], w[1])
                    .into_iter()
                    .map(|path| self.move_to_and_press(&path))
                    .collect_vec()
            })
            .collect_vec();

        // println!(
        //     "{:?}",
        //     cartesian_join_steps(h)
        //         .into_iter()
        //         .map(|p| p.into_iter().join(""))
        //         .collect_vec()
        // );

        cartesian_join_steps(h)
    }
}

fn cartesian_join_steps(steps: Vec<Vec<Path>>) -> Vec<Path> {
    steps
        .into_iter()
        .reduce(|le, ri| {
            le.into_iter()
                .cartesian_product(ri)
                .map(|(a, b)| join_paths(a, b))
                .collect_vec()
        })
        .unwrap()
}

fn join_paths(mut a: Path, b: Path) -> Path {
    a.extend(b);
    a
}

fn keep_shortest(mut paths: Vec<Path>) -> Vec<Path> {
    let min_len = paths.iter().map(|p| p.len()).min().unwrap();
    paths.retain(|p| p.len() == min_len);
    paths
}

fn path2str(path: &Path) -> String {
    path.into_iter().join("")
}

fn solve(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            println!("Considering {line}...");

            // let line = "029A";
            let numpad_path = once('A').chain(line.chars()).collect_vec();
            let dirpad_paths_1 = keep_shortest(NUMPAD.shortest_path_dirs_along(numpad_path));

            let dirpad_paths_2 = keep_shortest(
                dirpad_paths_1
                    .into_iter()
                    .flat_map(|dirpad_path| {
                        let dirpad_path = once('A').chain(dirpad_path.into_iter()).collect_vec();
                        keep_shortest(DIRPAD.shortest_path_dirs_along(dirpad_path))
                    })
                    .collect_vec(),
            );

            let all_dirpad_paths_3 = dirpad_paths_2
                .into_iter()
                .flat_map(|dirpad_path| {
                    let dirpad_path = once('A').chain(dirpad_path.into_iter()).collect_vec();
                    keep_shortest(DIRPAD.shortest_path_dirs_along(dirpad_path))
                })
                .collect_vec();

            let num = line[0..line.len() - 1].parse::<usize>().unwrap();
            let shortest_len = all_dirpad_paths_3.iter().map(|p| p.len()).min().unwrap();

            println!("  {} * {}", num, shortest_len);

            num * shortest_len
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
029A
980A
179A
456A
379A
"
        ),
        126384
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
