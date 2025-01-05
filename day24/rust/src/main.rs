use ast::Ast;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use std::{fs, time::Instant};
use z3::*;

fn main() {
    let input = include_str!("../../../inputs/input_24.txt");
    let input_manually_fixed = include_str!("../input_manually_fixed.txt");

    time(|| {
        // <1ms
        println!("First part: {}", solve(input));
    });

    time(|| {
        // a lot of manual work :P -- not machine-solved
        println!("Bonus: {}", bonus(input_manually_fixed));
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

fn bonus(input: &str) -> String {
    let size = 45;
    let (_, wires) = input.trim().split_once("\n\n").unwrap();
    let eqs = wires
        .lines()
        .map(|wire| {
            let (a, op, b, _, out) = wire.split(" ").collect_tuple().unwrap();
            (a, op, b, out)
        })
        .collect_vec();

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
        nodes.insert(format!("a{i:02}"), y.extract(i, i));
        nodes.insert(format!("c{i:02}"), z.extract(i, i));
    }

    // add equations, which define z as resulting from the computation graph
    let equations = eqs
        .iter()
        .map(|&(a, op, b, out)| {
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

    let these_equations_hold = ast::Bool::and(&ctx, &equations.iter().collect_vec());
    solver.assert(&these_equations_hold);

    if "generate dot graph text".contains(" ") {
        let mut nodes = FxHashSet::default();
        let mut edges = FxHashSet::default();

        // z
        for (a, op, b, out) in &eqs {
            nodes.insert((a.to_string(), a.to_string()));
            nodes.insert((b.to_string(), b.to_string()));
            nodes.insert((out.to_string(), out.to_string()));

            let opnode = format!("{a}_{op}_{b}");
            nodes.insert((opnode.clone(), op.to_string()));

            edges.insert((a.to_string(), opnode.clone()));
            edges.insert((b.to_string(), opnode.clone()));
            edges.insert((opnode.clone(), out.to_string()));
        }

        let nodes = nodes
            .into_iter()
            .map(|(id, label)| format!("{id} [label=\"{label}\"];"))
            .join("\n");

        let edges = edges
            .into_iter()
            .map(|(a, b)| format!("{a} -> {b};"))
            .join("\n");

        // Create the svg afterwards with this command (after installing graphviz):
        // dot -T svg computation_graph.dot -o computation_graph.svg
        fs::write(
            "./computation_graph.dot",
            format!("digraph computation_graph {{ {nodes} {edges} }}"),
        )
        .unwrap();
    }

    if "some old code" == "for the regular carry-add formulas" {
        let a = ast::BV::new_const(&ctx, "a", size); // x+y
        let c = ast::BV::new_const(&ctx, "c", size); // carry bits for computation of x+y

        // add equations for `a` as the result of regular addition
        // c_0 = 0
        solver.assert(
            &c.extract(0, 0)
                ._eq(&x.extract(0, 0).bvand(&y.extract(0, 0))),
        );
        // a_0 = x_0 XOR y_0
        solver.assert(
            &a.extract(0, 0)
                ._eq(&x.extract(0, 0).bvxor(&y.extract(0, 0))),
        );
        for i in 1..size {
            // c_{i>0} = (x_i AND y_i) OR (x_i AND c_(i-1)) OR (y_i AND c_(i-1))
            solver.assert(
                &c.extract(i, i)._eq(
                    &ast::BV::from_u64(&ctx, 0, 1)
                        .bvor(&x.extract(i, i).bvand(&y.extract(i, i)))
                        .bvor(&x.extract(i, i).bvand(&c.extract(i - 1, i - 1)))
                        .bvor(&y.extract(i, i).bvand(&c.extract(i - 1, i - 1))),
                ),
            );
            // a_{i>0} = x_i XOR y_i XOR c_(i-1)
            solver.assert(
                &a.extract(i, i)._eq(
                    &x.extract(i, i)
                        .bvxor(&y.extract(i, i).bvxor(&c.extract(i - 1, i - 1))),
                ),
            );
        }

        // indeed UNSAT, i.e. cannot prove that x+y != a, i.e. generally x+y == a
        for i in 0..size {
            println!(
                "x[..={i}] + y[..={i}] != a[..={i}] ? {:?}",
                solver.check_assumptions(&[x
                    .extract(i, 0)
                    .bvadd(&y.extract(i, 0))
                    ._eq(&a.extract(i, 0))
                    .not()])
            );
        }
        println!(
            "x + y != a ? {:?}",
            solver.check_assumptions(&[x.bvadd(&y)._eq(&a).not()])
        );
    }

    // SAT, i.e. can create x,y,z st x+y != z, i.e. NOT generally x+y == z
    for i in 0..size {
        println!(
            "x[..={i}] + y[..={i}] != z[..={i}] ? {:?}",
            solver.check_assumptions(&[x
                .extract(i, 0)
                .bvadd(&y.extract(i, 0))
                ._eq(&z.extract(i, 0))
                .not()])
        );
    }
    println!(
        "x + y != z ? {:?}",
        solver.check_assumptions(&[x.bvadd(&y)._eq(&z).not()])
    );

    // MANUAL :P
    "dgr,dtv,fgc,mtj,vvm,z12,z29,z37".to_string()
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
}
