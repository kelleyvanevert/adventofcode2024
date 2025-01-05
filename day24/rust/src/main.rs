use ast::Ast;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use rayon::prelude::*;
use std::time::Instant;
use z3::*;

fn main() {
    let input = include_str!("../../../inputs/input_24.txt");

    time(|| {
        // <1ms
        println!("First part: {}", solve(input));
    });

    time(|| {
        // ?
        println!("Bonus: {}", bonus(input, 46, "add"));
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

fn solve(input: &'static str) -> usize {
    let (init, wires) = input.trim().split_once("\n\n").unwrap();

    let mut values = FxHashMap::default();

    for line in init.lines() {
        let (node, val) = line.split_once(": ").unwrap();
        values.insert(node, val == "1");
    }

    type Equation = (&'static str, &'static str, &'static str);

    let mut equations = FxHashMap::default();

    for wire in wires.lines() {
        let (a, op, b, _, out) = wire.split(" ").collect_tuple().unwrap();
        equations.insert(out, (a, op, b));
    }

    fn eval(
        key: &'static str,
        values: &mut FxHashMap<&'static str, bool>,
        equations: &FxHashMap<&'static str, Equation>,
    ) -> bool {
        match values.get(key) {
            Some(val) => *val,
            None => {
                let &(a, op, b) = equations.get(key).unwrap();
                let av = eval(a, values, equations);
                let bv = eval(b, values, equations);
                let val = match op {
                    "AND" => av && bv,
                    "OR" => av || bv,
                    "XOR" => av != bv,
                    _ => unreachable!(),
                };
                values.insert(key, val);
                val
            }
        }
    }

    equations
        .keys()
        .filter(|k| k.starts_with("z"))
        .sorted()
        .enumerate()
        .map(|(i, &z)| (eval(&z, &mut values, &equations) as usize) << i)
        .sum()
}

fn bonus(input: &str, size: u32, goal: &str) -> String {
    let (_, wires) = input.trim().split_once("\n\n").unwrap();
    let eqs = wires
        .lines()
        .map(|wire| {
            let (a, op, b, _, out) = wire.split(" ").collect_tuple().unwrap();
            (a, op, b, out)
        })
        .collect_vec();

    let found_solution = (0..eqs.len())
        .tuple_combinations()
        // .flat_map(|(i, j)| {
        //     (0..eqs.len())
        //         .tuple_combinations()
        //         .filter(|&(k, l)| i != k && i != l && j != k && j != l)
        //         .map(|(k, l)| (i, j, k, l))
        //     // .collect_vec()
        // })
        .par_bridge()
        .find_map_any(|(i, j)| {
            (0..eqs.len()).tuple_combinations().find_map(|(k, l)| {
                if i == k || i == l || j == k || j == l {
                    return None;
                }

                println!("{i},{j},{k},{l}");

                let mut eqs = eqs.clone();
                (eqs[j].3, eqs[i].3) = (eqs[i].3, eqs[j].3);
                (eqs[l].3, eqs[k].3) = (eqs[k].3, eqs[l].3);

                let cfg = Config::new();
                let ctx = Context::new(&cfg);
                let solver = Solver::new(&ctx);

                let x = ast::BV::new_const(&ctx, "x", size);
                let y = ast::BV::new_const(&ctx, "y", size);
                let z = ast::BV::new_const(&ctx, "z", size);

                let mut nodes = FxHashMap::default();

                // add x's and y's and z's bit nodes
                for i in 0..size {
                    nodes.insert(format!("x{i:02}"), x.extract(i, i));
                    nodes.insert(format!("y{i:02}"), y.extract(i, i));
                    nodes.insert(format!("z{i:02}"), z.extract(i, i));
                }

                // add equations
                let equations = eqs
                    .into_iter()
                    .map(|(a, op, b, out)| {
                        if !nodes.contains_key(a) {
                            nodes.insert(a.to_string(), ast::BV::new_const(&ctx, a, 1));
                        }

                        if !nodes.contains_key(b) {
                            nodes.insert(b.to_string(), ast::BV::new_const(&ctx, b, 1));
                        }

                        if !nodes.contains_key(out) {
                            nodes.insert(out.to_string(), ast::BV::new_const(&ctx, out, 1));
                        }

                        let node_a = nodes.get(a).unwrap();
                        let node_b = nodes.get(b).unwrap();
                        let node_out = nodes.get(out).expect(&format!("should have out={out}"));

                        match &op[..] {
                            "AND" => node_a.bvand(&node_b)._eq(&node_out),
                            "OR" => node_a.bvor(&node_b)._eq(&node_out),
                            "XOR" => node_a.bvxor(&node_b)._eq(&node_out),
                            _ => unreachable!(),
                        }
                    })
                    .collect_vec();

                let equations_hold = &ast::Bool::and(&ctx, &equations.iter().collect_vec());

                // we want:
                //   equations AND
                //   for ALL x,y [ x + y = z ]

                // stated differently:
                // we want NO:
                //   equations AND
                //   x + y != z

                solver.assert(&ast::Bool::and(
                    &ctx,
                    &[
                        //
                        &equations_hold,
                        &(if goal == "and" {
                            ast::BV::bvand(&x, &y)
                        } else {
                            ast::BV::bvadd(&x, &y)
                        }
                        ._eq(&z)
                        .not()),
                    ],
                ));

                // solver.assert(&ast::Bool::implies(
                //     &equations_hold,
                //     &(if goal == "and" {
                //         ast::BV::bvand(&x, &y)
                //     } else {
                //         ast::BV::bvadd(&x, &y)
                //     }
                //     ._eq(&z)),
                // ));

                // println!("{}", solver);

                match solver.check() {
                    SatResult::Sat => {
                        // let model = solver.get_model().unwrap();
                        // println!("sat, with x = {}", model.eval(&x, true).unwrap());
                    }
                    SatResult::Unknown => {
                        println!("unknown?!")
                    }
                    SatResult::Unsat => {
                        println!("UNSAT :(");
                        println!(
                            "THIS MEANS WE FOUND A SOLUTION, crossing {i} -- {j}, and {k} -- {l}"
                        );
                        return Some((i, j, k, l));
                    }
                }

                None
            })
        });

    println!("");
    println!("===");
    println!("FOUND SOLUTION? {:?}", found_solution);

    // let cfg = Config::new();
    // let ctx = Context::new(&cfg);
    // let solver = Solver::new(&ctx);

    // let x = ast::BV::new_const(&ctx, "x", size);
    // let y = ast::BV::new_const(&ctx, "y", size);
    // let z = ast::BV::new_const(&ctx, "z", size);

    // let mut nodes = FxHashMap::default();

    // // add x's and y's and z's bit nodes
    // for i in 0..size {
    //     nodes.insert(format!("x{i:02}"), x.extract(i, i));
    //     nodes.insert(format!("y{i:02}"), y.extract(i, i));
    //     nodes.insert(format!("z{i:02}"), z.extract(i, i));
    // }

    // // add equations
    // let equations = wires
    //     .lines()
    //     .map(|wire| {
    //         let (a, op, b, _, out) = wire.split(" ").collect_tuple().unwrap();

    //         if !nodes.contains_key(a) {
    //             nodes.insert(a.to_string(), ast::BV::new_const(&ctx, a, 1));
    //         }

    //         if !nodes.contains_key(b) {
    //             nodes.insert(b.to_string(), ast::BV::new_const(&ctx, b, 1));
    //         }

    //         // if !nodes.contains_key(out) {
    //         //     nodes.insert(out.to_string(), ast::BV::new_const(&ctx, out, 1));
    //         // }

    //         let node_a = nodes.get(a).unwrap();
    //         let node_b = nodes.get(b).unwrap();
    //         let node_out = nodes.get(out).unwrap();

    //         match &op[..] {
    //             "AND" => node_a.bvand(&node_b)._eq(&node_out),
    //             "OR" => node_a.bvor(&node_b)._eq(&node_out),
    //             "XOR" => node_a.bvxor(&node_b)._eq(&node_out),
    //             _ => unreachable!(),
    //         }
    //     })
    //     .collect_vec();

    // let equations_hold = &ast::Bool::and(&ctx, &equations.iter().collect_vec());

    // // we want:
    // //   equations AND
    // //   for ALL x,y [ x + y = z ]

    // // stated differently:
    // // we want NO:
    // //   equations AND
    // //   x + y != z

    // solver.assert(&ast::Bool::and(
    //     &ctx,
    //     &[
    //         //
    //         &equations_hold,
    //         &(if goal == "and" {
    //             ast::BV::bvand(&x, &y)
    //         } else {
    //             ast::BV::bvadd(&x, &y)
    //         }
    //         ._eq(&z)
    //         .not()),
    //     ],
    // ));

    // // solver.assert(&ast::Bool::implies(
    // //     &equations_hold,
    // //     &(if goal == "and" {
    // //         ast::BV::bvand(&x, &y)
    // //     } else {
    // //         ast::BV::bvadd(&x, &y)
    // //     }
    // //     ._eq(&z)),
    // // ));

    // println!("{}", solver);

    // match solver.check() {
    //     SatResult::Sat => {
    //         let model = solver.get_model().unwrap();
    //         println!("sat, with x = {}", model.eval(&x, true).unwrap());
    //     }
    //     SatResult::Unknown => {
    //         panic!("unknown?>! :(")
    //     }
    //     SatResult::Unsat => {
    //         panic!("UNSAT :(")
    //     }
    // }

    "".to_string()
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"
        ),
        2024
    );

    assert_eq!(
        bonus(
            "
x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00
",
            6,
            "and",
        ),
        "z00,z01,z02,z05".to_string()
    );
}
